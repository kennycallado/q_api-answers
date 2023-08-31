CREATE TABLE IF NOT EXISTS answers ();

ALTER TABLE answers
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN question_id SERIAL NOT NULL,
  ADD COLUMN answer VARCHAR NOT NULL
  ;

INSERT INTO answers (question_id, answer) VALUES
  (1, '1'),
  (2, '1'),
  (3, '3'),
  (4, '5'),
  (5, '1')
  ;
