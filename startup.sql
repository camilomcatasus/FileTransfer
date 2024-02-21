DROP TABLE IF EXISTS Account;
CREATE TABLE IF NOT EXISTS Account (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    admin_access BOOLEAN NOT NULL,
    write_access BOOLEAN NOT NULL,
    read_access BOOLEAN NOT NULL
);

INSERT INTO Account (username, password, admin_access, write_access, read_access) VALUES
("admin", "yurt25#!", 1, 1, 1),
("reader", "yurt25#!", 0, 0, 1),
("writer", "yurt25#!", 0, 1, 1);
