table! {
    notes (note_id) {
        note_id -> Int4,
        title -> Varchar,
        content -> Nullable<Text>,
    }
}

table! {
    notes_tags (note_tag_id) {
        note_tag_id -> Int4,
        note_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    tags (tag_id) {
        tag_id -> Int4,
        name -> Varchar,
    }
}

joinable!(notes_tags -> notes (note_id));
joinable!(notes_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(notes, notes_tags, tags,);
