"""Test list append method"""

class Logger:
    def __init__(self):
        self.messages = []
    
    def log(self, msg: str) -> int:
        """Add a message to the log"""
        self.messages.append(msg)
        return len(self.messages)

def test_logger():
    logger = Logger()
    count = logger.log("Hello")
    return count