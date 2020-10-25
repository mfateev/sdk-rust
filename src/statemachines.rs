use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use crate::temporal::api::enums::v1::{CommandType, EventType};

#[derive(PartialEq, Eq, Hash)]
struct Transition<State: PartialEq + Eq + Hash + Debug + Copy, ExplicitEvent: PartialEq + Eq + Hash> {
    from: State,
    event: TransitionEvent<ExplicitEvent>,
}

#[derive(PartialEq, Eq, Hash)]
enum TransitionEvent<ExplicitEvent: PartialEq + Eq + Hash> {
    ExplicitEvent(ExplicitEvent),
    HistoryEvent(EventType),
    CommandEvent(CommandType),
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

struct StateMachineDefinition<State: PartialEq + Eq + Hash + Debug + Copy, ExplicitEvent: PartialEq + Eq + Hash, Data> {
    name: String,
    initial_state: State,
    final_states: Vec<State>,
    valid_event_types: HashSet<State>,
    transitions: HashMap<Transition<State, TransitionEvent<ExplicitEvent>>, TransitionAction<State, Data>>,
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

impl<State: PartialEq + Eq + Hash + Debug + Copy, ExplicitEvent: PartialEq + Eq + Hash, Data> StateMachineDefinition<State, ExplicitEvent, Data> {
    fn name(&self) -> &String {
        &self.name
    }

    fn initial_state(&self) -> &State {
        &self.initial_state
    }

    fn valid_event_types(&self) -> &HashSet<State> {
        &self.valid_event_types
    }

    fn get_transition_action(&self, transition: Transition<State, TransitionEvent<ExplicitEvent>>) -> &TransitionAction<State, Data> {
        return &self.transitions.get(&transition).expect("unknown transition");
    }
}