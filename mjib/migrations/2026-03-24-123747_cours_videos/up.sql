CREATE TABLE video_links (
    id SERIAL PRIMARY KEY,
    link TEXT NOT NULL
);

CREATE TABLE courses_video_links (
    id SERIAL PRIMARY KEY,
    course_id INTEGER NOT NULL,
    video_link_id INTEGER NOT NULL,
    FOREIGN KEY ("course_id") REFERENCES "courses"("id"),
    FOREIGN KEY ("video_link_id") REFERENCES "video_link"("id")
);