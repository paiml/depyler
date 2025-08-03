# Test async methods in classes

class AsyncCounter:
    def __init__(self, start: int = 0):
        self.value = start
    
    async def increment(self) -> int:
        await self._simulate_delay()
        self.value += 1
        return self.value
    
    async def get_value(self) -> int:
        return self.value
    
    async def _simulate_delay(self) -> None:
        # Simulated async operation
        pass

class AsyncDataProcessor:
    async def process(self, data: str) -> str:
        # Simulate async processing
        await self._async_work()
        return data.upper()
    
    async def _async_work(self) -> None:
        pass