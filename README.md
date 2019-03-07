# libnmtp - Neutral Money Transaction Protocol

NMTP is a high speed, decentralized payment network that emphasizes privacy and anonymity.

## About

libnmtp is the official library for the neutral money transaction protocol. While we do have a core implementation [here](https://github.com/neutralmoney/nmtpd). This is purely just an implementation of a node using this library.

We strive to create a fast and private means of sending money. 

## Documentation

Right now the wiki is sort of populated though it is mainly just a high level overview of the network. We plan to have a documentation page up shortly that overviews how each component works and how to use the library. 

## Features

**Datastore:** NMTP's datastore architecture allows for arbitrarily pruning and scales linearly with the amount of users. 

**No Fees:** Microtransactiona of any amount are completely possible with the lack of any fees. 

**High Speed Transactions:** NMTP is meant to maintain a high volume of transactions with low latency.

**Privacy and Anonymity:** NMTP has private transactions set by default.

**Create your own Token:** Tokens can be created on NMTP by a centralized authority or in a decentralized manner. 

**No funny business:** NMTP is not hosting an ICO, no premine, there isn't even an offically supported token by the core developers.

## FFI

Once the API is stabilized, bindings to the library will be released in many languages. 
