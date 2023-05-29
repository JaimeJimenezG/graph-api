-- Your SQL goes here
ALTER TABLE navigations ADD component_id INT NOT NULL DEFAULT 1;

ALTER TABLE navigations
  ADD CONSTRAINT navigations_component_id_fkey FOREIGN KEY (component_id)     
    REFERENCES components(id)