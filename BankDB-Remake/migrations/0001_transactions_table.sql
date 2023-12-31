create table transactions (
	username VARCHAR(20) NOT NULL PRIMARY KEY,
    operation VARCHAR(20) NOT NULL,
	amount DECIMAL(12,2) NOT NULL,
	recipient VARCHAR(20)
);