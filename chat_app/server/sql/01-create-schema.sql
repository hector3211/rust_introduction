CREATE TYPE todo_status_enum AS ENUM (
  'open'
  'close'
);

--TODOS
CREATE TABLE todo(
  id bigserial,
  cid bigint NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  title text Not NULL,status todo_status_enum NOT NULL DEAULT 'open'
);
ALTER SEQUENCE todo_id_seq RESTART WITH 1000;
