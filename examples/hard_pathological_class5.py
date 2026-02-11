# Pathological class pattern: State machine with transition table
# Tests: class simulating FSM with state tracking, history, event processing
# Workaround: avoid calling mutating self methods from other self methods
# (transpiler generates &self instead of &mut self). Process events inline instead.


class StateMachine:
    def __init__(self, initial_state: int) -> None:
        self.current: int = initial_state
        self.history: list[int] = [initial_state]
        self.transition_count: int = 0
        # Encode transitions as flat list: [from, event, to, from, event, to, ...]
        self.transitions: list[int] = []

    def add_transition(self, from_state: int, event: int, to_state: int) -> None:
        self.transitions.append(from_state)
        self.transitions.append(event)
        self.transitions.append(to_state)

    def process_event(self, event: int) -> bool:
        i: int = 0
        while i + 2 < len(self.transitions):
            if self.transitions[i] == self.current and self.transitions[i + 1] == event:
                self.current = self.transitions[i + 2]
                self.history.append(self.current)
                self.transition_count = self.transition_count + 1
                return True
            i = i + 3
        return False

    def get_state(self) -> int:
        return self.current

    def get_history_length(self) -> int:
        return len(self.history)

    def is_in_state(self, state: int) -> bool:
        return self.current == state

    def visited_state(self, state: int) -> bool:
        i: int = 0
        while i < len(self.history):
            if self.history[i] == state:
                return True
            i = i + 1
        return False


def test_module() -> int:
    passed: int = 0
    sm: StateMachine = StateMachine(0)
    # Build traffic light: 0=red, 1=green, 2=yellow
    sm.add_transition(0, 0, 1)  # red -> green on event 0
    sm.add_transition(1, 0, 2)  # green -> yellow
    sm.add_transition(2, 0, 0)  # yellow -> red
    sm.add_transition(1, 1, 0)  # emergency from green
    sm.add_transition(2, 1, 0)  # emergency from yellow
    # Test 1: initial state
    if sm.get_state() == 0:
        passed = passed + 1
    # Test 2: timer tick -> green
    sm.process_event(0)
    if sm.get_state() == 1:
        passed = passed + 1
    # Test 3: timer tick -> yellow
    sm.process_event(0)
    if sm.get_state() == 2:
        passed = passed + 1
    # Test 4: timer tick -> red
    sm.process_event(0)
    if sm.is_in_state(0) == True:
        passed = passed + 1
    # Test 5: history length (initial + 3 transitions = 4)
    if sm.get_history_length() == 4:
        passed = passed + 1
    # Test 6: visited state 2
    if sm.visited_state(2) == True:
        passed = passed + 1
    # Test 7: process event batch inline
    sm.process_event(0)  # red -> green
    sm.process_event(1)  # green -> red (emergency)
    if sm.get_state() == 0:
        passed = passed + 1
    # Test 8: transition count (3 + 2 = 5)
    if sm.transition_count == 5:
        passed = passed + 1
    return passed
