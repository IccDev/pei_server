-- Your SQL goes here
CREATE TABLE courses_disciplines (
  course_id INTEGER NOT NULL,
  discipline_id INTEGER NOT NULL,
  PRIMARY KEY("course_id", "discipline_id"),
  FOREIGN KEY ("course_id") REFERENCES "courses"("id"),
  FOREIGN KEY ("discipline_id") REFERENCES "disciplines"("id")
);