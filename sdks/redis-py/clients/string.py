from typing import Optional, List, Dict, Any

class StringClient:
    def __init__(self, client):
        self.client = client

    def get(self, key: str) -> Optional[str]:
        response = self.client._request("GET", f"/redis/string/{key}")
        return response.get("value") if response else None

    def set(self, key: str, value: str, ttl: Optional[int] = None) -> None:
        data = {"value": value}
        if ttl is not None:
            data["ttl"] = ttl
        self.client._request("POST", f"/redis/string/{key}", json=data)

    def delete(self, key: str) -> bool:
        response = self.client._request("DELETE", f"/redis/string/{key}")
        return response.get("deleted", False) if response else False

    def info(self, key: str) -> Optional[Dict[str, Any]]:
        response = self.client._request("GET", f"/redis/string/{key}/info")
        return response if response else None

    def batch_get(self, keys: List[str]) -> List[Optional[str]]:
        response = self.client._request("POST", "/redis/string/batch_get", json={"keys": keys})
        return response.get("values", []) if response else []

    def batch_set(self, operations: List[Dict[str, Any]]) -> None:
        self.client._request("POST", "/redis/string/batch_set", json={"operations": operations})
        