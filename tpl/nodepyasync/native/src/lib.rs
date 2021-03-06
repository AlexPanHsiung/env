use neon::prelude::*;
use pyo3::prelude::*;
use pyo3::types::*;
use pyo3::{py_run, wrap_pyfunction };

fn getPyRun( jstr: String )->String{
    let gil = Python::acquire_gil();
    let py = gil.python();
    let locals = PyDict::new(py);
    locals.set_item("myname","alex");

    py.run(
        r#"
def info( jstr ):
  import json
  obj = json.loads( jstr )
  return json.dumps(  obj )
  "#,
        None,
        Some(locals)).unwrap();

    //get method
    let max = locals.get_item("info").unwrap().to_object(py);
    let result = max.call1(py,( jstr, )).unwrap().extract::<String>(py).unwrap();
    format!("{}" , result )
}


struct MyTask{
    argument: String
}

impl Task for MyTask {
    type Output = String;
    type Error = ();
    type JsEvent = JsString;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        let ojstr = getPyRun( self.argument.clone()  );
        Ok(  ojstr )
    }

    fn complete<'a>(self, mut cx: TaskContext<'a>, result: Result<Self::Output, Self::Error>) -> JsResult<'_, Self::JsEvent> {
        Ok( cx.string( result.unwrap() ) )
    }
}

fn doSync(mut cx: FunctionContext) -> JsResult<JsString> {
    let jstr = cx.argument::<JsString>(0)?.value();
    let ojstr = getPyRun( jstr );

    Ok(cx.string(ojstr ))
}

fn doAsync(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let jstr = cx.argument::<JsString>(0)?.value();
    let cb = cx.argument::<JsFunction>(1)?;

    let task = MyTask { argument: ojstr };
    task.schedule( cb );

    Ok( cx.undefined())
}

//neon start
register_module!(mut cx, {
   cx.export_function("doSync" , doSync);
   cx.export_function("doAsync" , doAsync);
   Ok(())
});
