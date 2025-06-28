import sys
import os
import unittest
from unittest.mock import patch, MagicMock

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from client import DbxClient
from clients.string import StringClient

class TestStringClient(unittest.TestCase):
    def setUp(self):
        self.client = DbxClient(base_url="http://localhost:8080")
        self.string_client = StringClient(self.client)

    @patch("client.requests.request")
    def test_get(self, mock_request):
        mock_response = MagicMock()
        mock_response.json.return_value = {"value": "testvalue"}
        mock_response.content = True
        mock_response.raise_for_status = MagicMock()
        mock_request.return_value = mock_response

        result = self.string_client.get("testkey")
        self.assertEqual(result, "testvalue")
        mock_request.assert_called_once_with("GET", "http://localhost:8080/redis/string/testkey", timeout=None, headers={},)

    @patch("client.requests.request")
    def test_set(self, mock_request):
        mock_response = MagicMock()
        mock_response.content = False
        mock_response.raise_for_status = MagicMock()
        mock_request.return_value = mock_response

        self.string_client.set("testkey", "testvalue", ttl=3600)
        mock_request.assert_called_once_with(
            "POST",
            "http://localhost:8080/redis/string/testkey",
            timeout=None,
            headers={},
            json={"value": "testvalue", "ttl": 3600},
        )

    @patch("client.requests.request")
    def test_delete(self, mock_request):
        mock_response = MagicMock()
        mock_response.json.return_value = {"deleted": True}
        mock_response.content = True
        mock_response.raise_for_status = MagicMock()
        mock_request.return_value = mock_response

        result = self.string_client.delete("testkey")
        self.assertTrue(result)
        mock_request.assert_called_once_with("DELETE", "http://localhost:8080/redis/string/testkey", timeout=None, headers={})

    @patch("client.requests.request")
    def test_info(self, mock_request):
        mock_response = MagicMock()
        mock_response.json.return_value = {"ttl": 3600, "type": "string"}
        mock_response.content = True
        mock_response.raise_for_status = MagicMock()
        mock_request.return_value = mock_response

        result = self.string_client.info("testkey")
        self.assertEqual(result, {"ttl": 3600, "type": "string"})
        mock_request.assert_called_once_with("GET", "http://localhost:8080/redis/string/testkey/info", timeout=None, headers={})

    @patch("client.requests.request")
    def test_batch_get(self, mock_request):
        mock_response = MagicMock()
        mock_response.json.return_value = {"values": ["val1", "val2", None]}
        mock_response.content = True
        mock_response.raise_for_status = MagicMock()
        mock_request.return_value = mock_response

        keys = ["key1", "key2", "key3"]
        result = self.string_client.batch_get(keys)
        self.assertEqual(result, ["val1", "val2", None])
        mock_request.assert_called_once_with(
            "POST",
            "http://localhost:8080/redis/string/batch_get",
            timeout=None,
            headers={},
            json={"keys": keys},
        )

    @patch("client.requests.request")
    def test_batch_set(self, mock_request):
        mock_response = MagicMock()
        mock_response.content = False
        mock_response.raise_for_status = MagicMock()
        mock_request.return_value = mock_response

        operations = [
            {"key": "key1", "value": "val1"},
            {"key": "key2", "value": "val2", "ttl": 3600},
        ]
        self.string_client.batch_set(operations)
        mock_request.assert_called_once_with(
            "POST",
            "http://localhost:8080/redis/string/batch_set",
            timeout=None,
            headers={},
            json={"operations": operations},
        )

if __name__ == "__main__":
    unittest.main()