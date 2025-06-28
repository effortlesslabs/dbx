import requests
from typing import Optional, Any, Dict

from clients.string import StringClient
# Future imports: HashClient, SetClient, AdminClient

class DbxClient:
    def __init__(self, base_url: str, timeout: Optional[int] = None, headers: Optional[Dict[str, str]] = None):
        self.base_url = base_url.rstrip("/")
        self.timeout = timeout
        self.headers = headers or {}

        self.string = StringClient(self)
        # Initialize other clients here when implemented
        # self.hash = HashClient(self)
        # self.set = SetClient(self)
        # self.admin = AdminClient(self)

    def _request(self, method: str, path: str, **kwargs) -> Any:
        url = f"{self.base_url}{path}"
        try:
            response = requests.request(method, url, timeout=self.timeout, headers=self.headers, **kwargs)
            response.raise_for_status()
            if response.content:
                return response.json()
            return None
        except requests.RequestException as e:
            raise RuntimeError(f"Request failed: {e}") from e