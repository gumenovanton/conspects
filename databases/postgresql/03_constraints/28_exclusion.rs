#### EXCLUSION CONSTRAINTS
// like unique but more flexible and powerful
CREATE EXTENSION IF NOT EXISTS btree_gist; // we can't create bottom table without it

CREATE TABLE exclusion_constraints (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    room_id INTEGER,
    booking_status TEXT,
    reservation_period TSRANGE,
    EXCLUDE USING GIST (room_id WITH =, reservation_period WITH &&) WHERE (booking_status != 'cancelled')
        // here i said when inserting a new reservation
        // check if the new reservation period overlaps with existing ones
        // where booking_status = 'cancelled'
    );
