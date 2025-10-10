# Test async/await support
async def fetch_data(url: str) -> str:
    """Fetch data asynchronously."""
    result = await async_fetch(url)
    return result

async def process_batch(items: list[str]) -> list[str]:
    """Process items asynchronously."""
    results = []
    for item in items:
        data = await fetch_data(item)
        results.append(data)
    return results
