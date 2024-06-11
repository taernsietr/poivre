/// A type implementing this trait is able to convert its contents into a Vec<String>, in order to
/// be iterated over and turned into a HTML table row.
pub trait TableRow {
    fn headers(self) -> impl Iterator<Item = String>;
    fn into_row(self) -> impl Iterator<Item = String>;
}
