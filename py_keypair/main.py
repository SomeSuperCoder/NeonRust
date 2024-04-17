from ecdsa import SigningKey, VerifyingKey
import base58


class PublicKey:
    def __init__(self, inner: VerifyingKey):
        self.inner: VerifyingKey = inner

    def to_string(self) -> str:
        return base58.b58encode(self.inner.to_string()).decode()

    def from_string(self, source: str) -> PublicKey:
        return PublicKey(VerifyingKey.from_string(base58.b64decode(source).decode()))

    def verify(self, signature: str, message: str) -> bool:
        pass

class PrivateKey:
    def __init__(self, inner: SigningKey):
        self.inner: SigningKey = inner

    def to_pem(self) -> str:
        pass
    
    def from_pem(self, source: str) -> str:
        pass

    def public_key(self) -> PublicKey:
        return PublicKey(self.inner.verifying_key)
