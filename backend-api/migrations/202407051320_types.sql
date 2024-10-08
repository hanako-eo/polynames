CREATE TYPE ROLE AS ENUM('operator', 'spy', 'spectator');
CREATE TYPE COLOR AS ENUM('blue', 'gray', 'black');
CREATE TYPE IMAGE AS STRING;

CREATE TYPE PLAYER AS STRUCT(id UUID, role ROLE);
