CREATE TABLE IF NOT EXISTS users (
	id SERIAL PRIMARY KEY,
	name VARCHAR(50) NOT NULL,
	email VARCHAR(50) UNIQUE NOT NULL,
	password_hash TEXT NOT NULL,
	email_verified BOOLEAN DEFAULT FALSE, -- Add this line
	verification_token TEXT, -- Add this line
	password_reset_token TEXT,
	password_reset_expires_at TIMESTAMP,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

