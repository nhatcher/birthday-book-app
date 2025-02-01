use book::{Book, Friend};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
pub struct PyBook {
    book: Book,
}

#[pyclass]
#[derive(FromPyObject)]
pub struct PyFriend {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub email: String,
    #[pyo3(get, set)]
    pub last_name: String,
    #[pyo3(get, set)]
    pub birthday: Option<String>,
}

#[pyfunction]
fn create(owner: &str) -> PyBook {
    PyBook {
        book: Book::new(owner),
    }
}

#[pyfunction]
fn from_json_string(json_str: &str) -> PyResult<PyBook> {
    match Book::from_json_str(json_str) {
        Ok(book) => Ok(PyBook { book }),
        Err(s) => Err(PyValueError::new_err(s)),
    }
}

#[pyfunction]
fn read_from_file(file_path: &str) -> PyResult<PyBook> {
    match Book::read_from_file(file_path) {
        Ok(book) => Ok(PyBook { book }),
        Err(s) => Err(PyValueError::new_err(s)),
    }
}

#[pymethods]
impl PyBook {
    pub fn to_json_string(&self) -> PyResult<String> {
        self.book.to_json_string().map_err(PyValueError::new_err)
    }

    pub fn add_friend(&mut self, friend: &PyAny) -> PyResult<()> {
        let friend: &PyFriend = &friend.extract()?;
        let new_friend = Friend {
            name: friend.name.to_string(),
            last_name: friend.last_name.to_string(),
            email: friend.email.to_string(),
            birthday: friend.birthday.clone(),
        };
        self.book
            .add_friend(new_friend)
            .map_err(PyValueError::new_err)
    }

    pub fn write_to_file(&self, file_path: &str) -> PyResult<()> {
        self.book
            .write_to_file(file_path)
            .map_err(PyValueError::new_err)
    }

    pub fn get_first_friend_with_last_name(&self, last_name: &str) -> PyResult<PyFriend> {
        match &self.book.friends_with_last_name(last_name).get(0) {
            Some(friend) => Ok(PyFriend {
                name: friend.name.clone(),
                last_name: friend.last_name.clone(),
                email: friend.email.clone(),
                birthday: friend.birthday.clone(),
            }),
            None => Err(PyValueError::new_err("No friends".to_string())),
        }
    }

    pub fn get_owner(&self) -> String {
        self.book.owner()
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn pybook(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_function(wrap_pyfunction!(from_json_string, m)?)?;
    m.add_function(wrap_pyfunction!(create, m)?)?;
    m.add_function(wrap_pyfunction!(read_from_file, m)?)?;
    Ok(())
}
