# crypto

Crypto algorithms from scratch. Academic purposes only; Cryptoanalysis work; 
Notes and research

## Crypto systems implementations 

The implementations of these crypto primitives are for academical purposes only.
Highly inspired by [@arnaucube cryptofun project](https://github.com/arnaucube/cryptofun)

### RSA cryptosystem & Blind signature & Homomorphic Multiplication
- https://en.wikipedia.org/wiki/RSA_(cryptosystem)#
- https://en.wikipedia.org/wiki/Blind_signature
- https://en.wikipedia.org/wiki/Homomorphic_encryption

- [ ] GenerateKeyPair
- [ ] Encrypt
- [ ] Decrypt
- [ ] Blind
- [ ] Blind Signature
- [ ] Unblind Signature- RSA- RSA  
- [ ] Verify Signature
- [ ] Homomorphic Multiplication

### Paillier cryptosystem & Homomorphic Addition
- https://en.wikipedia.org/wiki/Paillier_cryptosystem
- https://en.wikipedia.org/wiki/Homomorphic_encryption

- [ ] GenerateKeyPair
- [ ] Encrypt
- [ ] Decrypt
- [ ] Homomorphic Addition

### Shamir Secret Sharing
- https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing

- [ ] create secret sharing from number of secrets needed, number of shares,
  random point p, secret to share
- [ ] Lagrange Interpolation to restore the secret from the shares

### Diffie-Hellman
- https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange

- [ ] key exchange

### ECC
- https://en.wikipedia.org/wiki/Elliptic-curve_cryptography

- [ ] define elliptic curve
- [ ] get point at X
- [ ] get order of a Point on the elliptic curve
- [ ] Add two points on the elliptic curve
- [ ] Multiply a point n times on the elliptic curve

### ECC ElGamal
- https://en.wikipedia.org/wiki/ElGamal_encryption

- [ ] ECC ElGamal key generation
- [ ] ECC ElGamal Encrypton
- [ ] ECC ElGamal Decryption

### ECC ECDSA
- https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm

- [ ] define ECDSA data structure
- [ ] ECDSA Sign
- [ ] ECDSA Verify signature


### Schnorr signature
- https://en.wikipedia.org/wiki/Schnorr_signature

- [ ] Hash[M || R] (where M is the msg bytes and R is a Point on the ECC, using
  sha256 hash function)
- [ ] Generate Schnorr scheme
- [ ] Sign
- [ ] Verify signature


### Bn128
This is implemented followng the implementations and info from:
- https://github.com/iden3/zksnark
- https://github.com/zcash/zcash/tree/master/src/snark
- `Multiplication and Squaring on Pairing-Friendly
Fields`, Augusto Jun Devegili, Colm Ó hÉigeartaigh, Michael Scott, and Ricardo
Dahab
https://pdfs.semanticscholar.org/3e01/de88d7428076b2547b60072088507d881bf1.pdf
- `Optimal Pairings`, Frederik Vercauteren
  https://www.cosic.esat.kuleuven.be/bcrypt/optimal.pdf
- `Double-and-Add with Relative Jacobian
Coordinates`, Björn Fay https://eprint.iacr.org/2014/1014.pdf
- `Fast and Regular Algorithms for Scalar Multiplication
over Elliptic Curves`, Matthieu Rivain https://eprint.iacr.org/2011/338.pdf

- [ ] Fq, Fq2, Fq6, Fq12 operations
- [ ] G1, G2 operations

