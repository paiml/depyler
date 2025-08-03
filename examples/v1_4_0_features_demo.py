# Depyler v1.4.0 - Async/Await Support Demo

# 1. Basic async function
async def hello_async() -> str:
    return "Hello from async!"

# 2. Async function with await
async def fetch_user(user_id: int) -> dict[str, str]:
    # Simulate async database fetch
    await async_sleep(0.1)
    return {"id": str(user_id), "name": f"User{user_id}"}

async def async_sleep(seconds: float) -> None:
    # Placeholder for actual async sleep
    pass

# 3. Async methods in classes
class AsyncService:
    def __init__(self, name: str):
        self.name = name
        self.requests_count = 0
    
    async def handle_request(self, request_id: int) -> str:
        self.requests_count += 1
        result = await self._process(request_id)
        return f"{self.name} processed: {result}"
    
    async def _process(self, request_id: int) -> str:
        await async_sleep(0.05)
        return f"Request#{request_id}"

# 4. Async with multiple awaits
async def process_batch(items: list[int]) -> list[str]:
    results = []
    for item in items:
        user = await fetch_user(item)
        result = f"Processed {user['name']}"
        results.append(result)
    return results

# 5. Async main function
async def main() -> None:
    # Test basic async
    greeting = await hello_async()
    print(greeting)
    
    # Test async class methods
    service = AsyncService("API-Service")
    response = await service.handle_request(123)
    print(response)
    
    # Test batch processing
    items = [1, 2, 3, 4, 5]
    batch_results = await process_batch(items)
    for result in batch_results:
        print(result)