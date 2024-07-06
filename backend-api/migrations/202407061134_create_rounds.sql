CREATE SEQUENCE IF NOT EXISTS round_id_sequence START 1;
CREATE TABLE IF NOT EXISTS rounds (
    id INT PRIMARY KEY DEFAULT nextval('round_id_sequence'),
    id_game UUID NOT NULL,
    clue STRING,
    nb_cards_to_find INT DEFAULT 0,

    FOREIGN KEY (id_game) REFERENCES games (id),
);
