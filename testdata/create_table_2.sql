CREATE TABLE dummies (
    face_width INTEGER NOT NULL,
    notes VARCHAR NULL,
    as_of_date DATE NOT NULL,
    synthetic_pk VARCHAR NOT NULL
);

ALTER TABLE dummies ADD CONSTRAINT dummies_pk PRIMARY KEY (synthetic_pk);
