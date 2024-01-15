-- Create a trigger function to update last_transaction in clients
CREATE OR REPLACE FUNCTION update_last_transaction()
RETURNS TRIGGER AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM clients WHERE username = NEW.username) THEN
        RAISE EXCEPTION 'No row found in clients with username: %', NEW.username;
    END IF;

    IF NEW.operation = 'transfer' AND NEW.recipient IS NULL THEN
        RAISE EXCEPTION 'Column "recipient" cannot be null for an operation of type "transfer"';
    END IF;

    IF NEW.recipient IS NOT NULL AND NOT EXISTS (SELECT 1 FROM clients WHERE username = NEW.recipient) THEN
        RAISE EXCEPTION 'No row found in clients with username specified for recipient: %', NEW.recipient;
    END IF;

    UPDATE clients
    SET last_transaction = NEW.username
    WHERE username = NEW.username;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger that calls the update_last_transaction function
CREATE TRIGGER update_clients_last_transaction
AFTER INSERT OR UPDATE ON transactions
FOR EACH ROW
EXECUTE FUNCTION update_last_transaction();