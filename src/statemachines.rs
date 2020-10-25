use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use crate::temporal::api::enums::v1::{CommandType, EventType};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Transition<State: PartialEq + Eq + Hash + Debug + Copy, ExplicitEvent: PartialEq + Eq + Hash + Debug + Copy> {
    from: State,
    event: TransitionEvent<ExplicitEvent>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum TransitionEvent<ExplicitEvent: PartialEq + Eq + Hash + Debug + Copy> {
    Explicit(ExplicitEvent),
    History(EventType),
    Command(CommandType),
}

struct DynamicTransitionAction<State: PartialEq + Eq + Hash + Debug + Copy, Data> {
    callback: fn(Data) -> State,
    expected_states: HashSet<State>,
}

struct FixedTransitionAction<State: PartialEq + Eq + Hash + Debug + Copy, Data> {
    callback: fn(Data),
    state: State,
}

enum TransitionAction<State: PartialEq + Eq + Hash + Debug + Copy, Data> {
    Dynamic(DynamicTransitionAction<State, Data>),
    Fixed(FixedTransitionAction<State, Data>),
}

struct StateMachineDefinition<State: PartialEq + Eq + Hash + Debug + Copy, ExplicitEvent: PartialEq + Eq + Hash + Debug + Copy, Data> {
    name: String,
    initial_state: State,
    final_states: Vec<State>,
    valid_event_types: HashSet<State>,
    transitions: HashMap<Transition<State, TransitionEvent<ExplicitEvent>>, TransitionAction<State, Data>>,
}

impl<State: PartialEq + Eq + Hash + Debug + Copy, ExplicitEvent: PartialEq + Eq + Hash + Debug + Copy> Transition<State, ExplicitEvent> {

    fn new(from: State, event: TransitionEvent<ExplicitEvent>) -> Transition<State, ExplicitEvent> {
        Transition { from, event }
    }
}

impl<State: PartialEq + Eq + Hash + Debug + Copy, Data> FixedTransitionAction<State, Data> {

    fn new(callback: fn(Data), state: State) -> FixedTransitionAction<State, Data> {
        FixedTransitionAction { callback, state }
    }
}

impl<State: PartialEq + Eq + Hash + Debug + Copy, Data> DynamicTransitionAction<State, Data> {

    fn new(callback: fn(Data) -> State, expected_states: HashSet<State>) -> DynamicTransitionAction<State, Data> {
        DynamicTransitionAction { callback, expected_states }
    }
}

impl<State: PartialEq + Eq + Hash + Debug + Copy, Data> DynamicTransitionAction<State, Data> {

    fn apply(&self, data: Data) -> State {
        let result = (self.callback)(data);
        if !self.expected_states.contains(&result) {
            panic!("{:?} state is not expected. Expected states are {:?}", result, self.expected_states);
        }
        result
    }
}

impl<State: PartialEq + Eq + Hash + Debug + Copy, Data> FixedTransitionAction<State, Data> {

    fn apply(&self, data: Data) -> State {
        (self.callback)(data);
        self.state
    }
}

impl<State: PartialEq + Eq + Hash + Debug + Copy, ExplicitEvent: PartialEq + Eq + Hash + Debug + Copy, Data> StateMachineDefinition<State, ExplicitEvent, Data> {

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn initial_state(&self) -> &State {
        &self.initial_state
    }

    pub fn valid_event_types(&self) -> &HashSet<State> {
        &self.valid_event_types
    }

    pub fn get_transition_action(&self, transition: Transition<State, TransitionEvent<ExplicitEvent>>) -> &TransitionAction<State, Data> {
        return &self.transitions.get(&transition).expect("unknown transition");
    }

    pub fn add_explicit(&mut self, from: State, explicit_event: ExplicitEvent, to: State, action: fn(Data)) -> &mut Self {
        self.check_final_state(from);
        let mut event: TransitionEvent<ExplicitEvent>;
        event = TransitionEvent::Explicit(explicit_event);
        self.add_impl(Transition::new(from, TransitionEvent::Explicit(event)), TransitionAction::Fixed(FixedTransitionAction::new(action, to)));
        self
    }

    fn add_impl(&mut self, transition: Transition<State, TransitionEvent<ExplicitEvent>>, target: TransitionAction<State, Data>) {
        if self.transitions.contains_key(&transition) {
            panic!("Duplicated transition not allowed");
        }
        self.transitions.insert(transition, target);
    }

    fn check_final_state(&self, from: State) {
        if self.final_states.contains(&from) {
            panic!("State transition from a final state is not allowed");
        }
    }
}