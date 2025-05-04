CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    full_name TEXT NOT NULL,
    cpf_cnpj TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    user_type TEXT CHECK (user_type IN ('common', 'shopkeeper')) NOT NULL
);

CREATE TABLE wallets (
    id SERIAL PRIMARY KEY,
    user_id INTEGER UNIQUE NOT NULL REFERENCES users(id),
    balance DECIMAL NOT NULL DEFAULT 0
);

CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    payer_id INTEGER NOT NULL REFERENCES users(id),
    payee_id INTEGER NOT NULL REFERENCES users(id),
    value DECIMAL NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users (id, full_name, cpf_cnpj, email, password, user_type)
VALUES
  (1, 'João Comprador', '12345678901', 'joao@exemplo.com', 'senha123', 'common'),
  (2, 'Maria Vendedora', '98765432100', 'maria@exemplo.com', 'senha456', 'shopkeeper');