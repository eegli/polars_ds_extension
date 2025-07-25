#![allow(non_snake_case)]
/// Linear Regression Interop with Python
use crate::linear::{
    lr::{
        lr_solvers::{ElasticNet, LR},
        LinearModel,
    },
    online_lr::lr_online_solvers::OnlineLR,
    LinalgErrors,
};
use crate::utils::interop::{IntoFaer, IntoNdarray};
use numpy::{IntoPyArray, PyArray1, PyArray2, PyArrayMethods, PyReadonlyArray1, PyReadonlyArray2};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

impl From<LinalgErrors> for PyErr {
    fn from(value: LinalgErrors) -> Self {
        PyValueError::new_err(value.to_string())
    }
}

#[pyclass(subclass)]
pub struct PyLR {
    lr: LR<f64>,
}

#[pymethods]
impl PyLR {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature=(
        solver = "qr",
        lambda_ = 0.,
        add_bias = false,
    ))]
    pub fn new(solver: &str, lambda_: f64, add_bias: bool) -> Self {
        PyLR {
            lr: LR::new(solver, lambda_, add_bias),
        }
    }

    pub fn is_fit(&self) -> bool {
        self.lr.is_fit()
    }

    pub fn fit(&mut self, X: PyReadonlyArray2<f64>, y: PyReadonlyArray2<f64>) -> PyResult<()> {
        let arr = X.as_raw_array();
        let x = X.into_faer();
        let y = y.into_faer();
        match self.lr.fit(x, y) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn set_coeffs_and_bias(
        &mut self,
        coeffs: PyReadonlyArray1<f64>,
        bias: f64,
    ) -> PyResult<()> {
        match coeffs.as_slice() {
            Ok(s) => Ok(self.lr.set_coeffs_and_bias(s, bias)),
            Err(e) => Err(e.into()),
        }
    }

    pub fn predict<'py>(
        &self,
        py: Python<'py>,
        X: PyReadonlyArray2<f64>,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let x = X.into_faer();
        match self.lr.predict(x) {
            Ok(result) => {
                // result should be n by 1, where n = x.nrows()
                let res = result.col_as_slice(0);
                let v = res.to_vec();
                Ok(v.into_pyarray(py))
            }
            Err(e) => Err(e.into()),
        }
    }

    #[getter]
    pub fn coeffs<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray1<f64>>> {
        match self.lr.coeffs_as_vec() {
            Ok(v) => Ok(v.into_pyarray(py)),
            Err(e) => Err(e.into()),
        }
    }

    #[getter]
    pub fn bias(&self) -> f64 {
        self.lr.bias()
    }

    #[getter]
    pub fn lambda_(&self) -> f64 {
        self.lr.lambda
    }
}

#[pyclass(subclass)]
pub struct PyElasticNet {
    lr: ElasticNet<f64>,
}

#[pymethods]
impl PyElasticNet {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature=(
        l1_reg,
        l2_reg,
        add_bias = false,
        tol = 1e-5,
        max_iter = 2000,
    ))]
    pub fn new(l1_reg: f64, l2_reg: f64, add_bias: bool, tol: f64, max_iter: usize) -> Self {
        PyElasticNet {
            lr: ElasticNet::new(l1_reg, l2_reg, add_bias, tol, max_iter),
        }
    }

    pub fn set_coeffs_and_bias(
        &mut self,
        coeffs: PyReadonlyArray1<f64>,
        bias: f64,
    ) -> PyResult<()> {
        match coeffs.as_slice() {
            Ok(s) => Ok(self.lr.set_coeffs_and_bias(s, bias)),
            Err(e) => Err(e.into()),
        }
    }

    pub fn is_fit(&self) -> bool {
        self.lr.is_fit()
    }

    pub fn fit(&mut self, X: PyReadonlyArray2<f64>, y: PyReadonlyArray2<f64>) -> PyResult<()> {
        let x = X.into_faer();
        let y = y.into_faer();
        match self.lr.fit(x, y) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn predict<'py>(
        &self,
        py: Python<'py>,
        X: PyReadonlyArray2<f64>,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let x = X.into_faer();
        match self.lr.predict(x) {
            Ok(result) => {
                // result should be n by 1, where n = x.nrows()
                let res = result.col_as_slice(0);
                let v = res.to_vec();
                Ok(v.into_pyarray(py))
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn add_bias(&self) -> bool {
        self.lr.add_bias()
    }

    #[getter]
    pub fn coeffs<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray1<f64>>> {
        match self.lr.coeffs_as_vec() {
            Ok(v) => Ok(v.into_pyarray(py)),
            Err(e) => Err(e.into()),
        }
    }

    #[getter]
    pub fn bias(&self) -> f64 {
        self.lr.bias()
    }

    #[getter]
    pub fn regularizers(&self) -> (f64, f64) {
        self.lr.regularizers()
    }
}

#[pyclass(subclass)]
pub struct PyOnlineLR {
    lr: OnlineLR<f64>,
}

#[pymethods]
impl PyOnlineLR {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature=(lambda_=0., add_bias=false))]
    pub fn new(lambda_: f64, add_bias: bool) -> Self {
        PyOnlineLR {
            lr: OnlineLR::new(lambda_, add_bias),
        }
    }

    pub fn is_fit(&self) -> bool {
        self.lr.is_fit()
    }

    pub fn fit(&mut self, X: PyReadonlyArray2<f64>, y: PyReadonlyArray2<f64>) -> PyResult<()> {
        let x = X.into_faer();
        let y = y.into_faer();
        match self.lr.fit(x, y) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn update(&mut self, X: PyReadonlyArray2<f64>, y: PyReadonlyArray2<f64>, c: f64) {
        let x = X.into_faer();
        let y = y.into_faer();
        self.lr.update(x, y, c);
    }

    pub fn set_coeffs_bias_inverse(
        &mut self,
        coeffs: PyReadonlyArray1<f64>,
        bias: f64,
        inv: PyReadonlyArray2<f64>,
    ) -> PyResult<()> {
        match coeffs.as_slice() {
            Ok(s) => match self.lr.set_coeffs_bias_inverse(s, bias, inv.into_faer()) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.into()),
            },
            Err(e) => Err(e.into()),
        }
    }

    pub fn predict<'py>(
        &self,
        py: Python<'py>,
        X: PyReadonlyArray2<f64>,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let x = X.into_faer();
        match self.lr.predict(x) {
            Ok(result) => {
                // result should be n by 1, where n = x.nrows()
                let res = result.col_as_slice(0);
                let v = res.to_vec();
                Ok(v.into_pyarray(py))
            }
            Err(e) => Err(e.into()),
        }
    }

    #[getter]
    pub fn coeffs<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray1<f64>>> {
        match self.lr.coeffs_as_vec() {
            Ok(v) => Ok(v.into_pyarray(py)),
            Err(e) => Err(e.into()),
        }
    }

    #[getter]
    pub fn inv<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray2<f64>>> {
        match self.lr.get_inv() {
            Ok(matrix) => {
                let mat = matrix.into_ndarray().to_owned();
                Ok(mat.into_pyarray(py))
            }
            Err(e) => Err(e.into()),
        }
    }

    #[getter]
    pub fn bias(&self) -> f64 {
        self.lr.bias()
    }

    #[getter]
    pub fn lambda_(&self) -> f64 {
        self.lr.lambda
    }
}
