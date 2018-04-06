-- Set DB schema , target postgres sql 9.4
-- Ensure that DB name matches ! 

-- Due to importer limitation, can't have ; anywhere else than end of query. my bad ;)

CREATE TABLE garden (
    garden_id SERIAL PRIMARY KEY
    , name varchar(100) 
    , last_update timestamp with time zone
    , next_update timestamp with time zone
    , parcels json
    , plants json
);

