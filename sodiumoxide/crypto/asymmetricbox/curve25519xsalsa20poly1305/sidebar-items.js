initSidebarItems({"struct":[["Nonce","`Nonce` for asymmetric authenticated encryption"],["PrecomputedKey","Applications that send several messages to the same receiver can gain speed by splitting `seal()` into two steps, `precompute()` and `seal_precomputed()`. Similarly, applications that receive several messages from the same sender can gain speed by splitting `open()` into two steps, `precompute()` and `open_precomputed()`."],["PublicKey","`PublicKey` for asymmetric authenticated encryption"],["SecretKey","`SecretKey` for asymmetric authenticated encryption"]],"fn":[["gen_keypair","`gen_keypair()` randomly generates a secret key and a corresponding public key."],["gen_nonce","`gen_nonce()` randomly generates a nonce"],["open","`open()` verifies and decrypts a ciphertext `c` using the receiver's secret key `sk`, the senders public key `pk`, and a nonce `n`. It returns a plaintext `Some(m)`. If the ciphertext fails verification, `open()` returns `None`."],["open_precomputed","`open_precomputed()` verifies and decrypts a ciphertext `c` using a precomputed key `k` and a nonce `n`. It returns a plaintext `Some(m)`. If the ciphertext fails verification, `open_precomputed()` returns `None`."],["precompute","`precompute()` computes an intermediate key that can be used by `seal_precomputed()` and `open_precomputed()`"],["seal","`seal()` encrypts and authenticates a message `m` using the senders secret key `sk`, the receivers public key `pk` and a nonce `n`. It returns a ciphertext `c`."],["seal_precomputed","`seal_precomputed()` encrypts and authenticates a message `m` using a precomputed key `k`, and a nonce `n`. It returns a ciphertext `c`."]],"constant":[["NONCEBYTES",""],["PRECOMPUTEDKEYBYTES",""],["PUBLICKEYBYTES",""],["SECRETKEYBYTES",""]]});