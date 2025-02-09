# Simple Payment Engine
## Intro
This is a simple payment application written in Rust that processes a CSV file and ouptut the summary of all the accounts

## Design
The application follows a simple pattern of reading the CSV file sequentially, stores all the transactions per client and then parses all clients in parallel

* To parallelize the processing, I used rayon.
* To deserialize the data, I used serde
* I used different types for amounts, transaction identifiers and clients to benefit from Rust strong typing.
* The application writes the result in the stdout and write errors to stderr.


## Testing

The testing follows on an integration level approach. 
Several test files can be found under ```test_resources``` folder. 
The tests read the file, parse the content and check the result (after reordering) to ensure that the results are as expected.
The verification in itself can be found in the ```tests``` folder.


## Assumptions
* The transactions are stored in sequence of time
* Duplicated transactions should not be processed
* Duplicated contests should not be processed
* Locked accounts mean something is wrong with the account, but payment records still need to be summarized

## Possible improvements
* The payment records could be streamlined into different processors (per account) in order to parallelize the client summaries. Tokio and async could be helpful 
* Depending on hardware to be used, the number of rayon workers could be better controlled with specific values
* A more interesting approach for tests could be used, by having test split into numbers(or folders) and having always an input/output described for each, so adding more tests would be done only by adding new input/output files. 