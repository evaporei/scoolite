var N = null;var searchIndex = {};
searchIndex["scoolite"]={"doc":"","items":[[3,"Row","scoolite","This struct contains the first table available for storage. It will be defined by the user eventually.",N,N],[12,"id","","",0,N],[12,"username","","",0,N],[12,"email","","",0,N],[3,"Table","","In memory storage of `Row`s.",N,N],[12,"rows","","",1,N],[4,"Error","","Main `Error` type, each `enum` variant represents a different error.",N,N],[13,"UnrecognizedStatement","","",2,N],[13,"SyntaxError","","",2,N],[5,"build_command","","This function is just a proxy that creates a `Command` or returns an `Error`. The way it decides if it will return a `MetaCommand` or a `Statement` is by looking on the `String` `input` if it starts with a dot (`.`).",N,[[["str"]],["result",["box","error"]]]],[5,"print_prompt","","Just prints `db >` to the terminal.",N,[[]]],[5,"read_input","","This function flushes the stdout, so that no outputs to the terminal get's stuck. Then it reads a line from stdin and returns it as a `String`.",N,[[],["string"]]],[5,"print_error","","Just prints an `Error`'s `description` to the terminal.",N,[[["error"]]]],[0,"as_any","","Module for `AsAny` trait",N,N],[8,"AsAny","scoolite::as_any","This trait has the purpose of casting between trait objects and Sized structures/enums.",N,N],[10,"as_any","","",3,[[["self"]],["any"]]],[0,"command","scoolite","Module for `Command`s/`Statement`s of the SQL-like language",N,N],[4,"MetaCommand","scoolite::command","`MetaCommand` is the `enum` that contains all meta commands for `scoolite`. An example of meta command is `.exit`, it does not belong to the `SQL` specification however it is used to close the program/REPL.",N,N],[13,"Exit","","",4,N],[4,"Statement","","`Statement` is the `enum` that contains all of the statements for `scoolite`. An example of a statement is `insert`, it does belong to the `SQL` specification and it is used to add a row to a table.",N,N],[13,"Insert","","",5,N],[13,"Select","","",5,N],[5,"build_command","","This function is just a proxy that creates a `Command` or returns an `Error`. The way it decides if it will return a `MetaCommand` or a `Statement` is by looking on the `String` `input` if it starts with a dot (`.`).",N,[[["str"]],["result",["box","error"]]]],[5,"build_not_implemented_error","","Creates an `Error` with the default `\"not implemented\"` message.",N,[[["str"]],["error"]]],[8,"Command","","The interface that every `Command` asks for is just an `execute` method, which executes the specific logic for the `Command`.",N,N],[10,"execute","","",6,[[["self"],["table"]],["result",["error"]]]],[11,"from_str","","Tries to parse an `&str` `input` into a `Box<Command>`, if it fails it returns a `\"not implemented error\"` `Error`.",4,[[["str"]],["result",["box","error"]]]],[11,"from_str","","Tries to parse an `&str` `input` into a `Box<Command>`, if it fails it returns a `\"not implemented error\"` `Error`.",5,[[["str"]],["result",["box","error"]]]],[11,"insert","","Creates a new `Row` based of an `input` `&str` and inserts it inside of a `table`. This is what get's called when something like `Statement::Insert(\"insert 1 john john@mailbox.com\").execute()` happens.",5,[[["self"],["str"],["table"]],["result",["error"]]]],[11,"select","","Prints all `Row`s inside of a table. This is what get's called when something like `Statement::Select.execute()` happens.",5,[[["self"],["table"]],["result",["error"]]]],[0,"error","scoolite","Module for `Error` type",N,N],[4,"Error","scoolite::error","Main `Error` type, each `enum` variant represents a different error.",N,N],[13,"UnrecognizedStatement","","",2,N],[13,"SyntaxError","","",2,N],[11,"get_description","scoolite","This function just get's the description depending of each variant of the `Error` type, usually the first field.",2,[[["self"]],["str"]]],[0,"io","","Module for interacting with stdio and stdout",N,N],[5,"print_prompt","scoolite::io","Just prints `db >` to the terminal.",N,[[]]],[5,"read_input","","This function flushes the stdout, so that no outputs to the terminal get's stuck. Then it reads a line from stdin and returns it as a `String`.",N,[[],["string"]]],[5,"print_error","","Just prints an `Error`'s `description` to the terminal.",N,[[["error"]]]],[0,"table","scoolite","Module for `Table` type",N,N],[3,"Table","scoolite::table","In memory storage of `Row`s.",N,N],[12,"rows","","",1,N],[11,"new","scoolite","Creates a new table.",1,[[],["self"]]],[11,"add_row","","Adds a `Row` into the `rows` `Vec`.",1,[[["self"],["row"]]]],[11,"list_rows","","Returns a reference to all `Row`s inside the table.",1,[[["self"]],["vec"]]],[0,"row","","Module for `Row` type",N,N],[3,"Row","scoolite::row","This struct contains the first table available for storage. It will be defined by the user eventually.",N,N],[12,"id","","",0,N],[12,"username","","",0,N],[12,"email","","",0,N],[11,"from_str","scoolite","Receives an input like `1 john john@mailbox.com` and it builds a `Row` with these fields. If any errors happen on the parse step, it will return an `Error`.",0,[[["str"]],["result",["error"]]]],[8,"AsAny","","This trait has the purpose of casting between trait objects and Sized structures/enums.",N,N],[10,"as_any","","",3,[[["self"]],["any"]]],[8,"Command","","The interface that every `Command` asks for is just an `execute` method, which executes the specific logic for the `Command`.",N,N],[10,"execute","","",6,[[["self"],["table"]],["result",["error"]]]],[11,"into","","",0,[[["self"]],["u"]]],[11,"to_string","","",0,[[["self"]],["string"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"get_type_id","","",0,[[["self"]],["typeid"]]],[11,"try_into","","",0,[[["self"]],["result"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"into","","",1,[[["self"]],["u"]]],[11,"from","","",1,[[["t"]],["t"]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"get_type_id","","",1,[[["self"]],["typeid"]]],[11,"try_into","","",1,[[["self"]],["result"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"into","","",2,[[["self"]],["u"]]],[11,"to_string","","",2,[[["self"]],["string"]]],[11,"to_owned","","",2,[[["self"]],["t"]]],[11,"clone_into","","",2,N],[11,"from","","",2,[[["t"]],["t"]]],[11,"try_from","","",2,[[["u"]],["result"]]],[11,"borrow","","",2,[[["self"]],["t"]]],[11,"get_type_id","","",2,[[["self"]],["typeid"]]],[11,"try_into","","",2,[[["self"]],["result"]]],[11,"borrow_mut","","",2,[[["self"]],["t"]]],[11,"into","scoolite::command","",4,[[["self"]],["u"]]],[11,"from","","",4,[[["t"]],["t"]]],[11,"try_from","","",4,[[["u"]],["result"]]],[11,"borrow","","",4,[[["self"]],["t"]]],[11,"get_type_id","","",4,[[["self"]],["typeid"]]],[11,"try_into","","",4,[[["self"]],["result"]]],[11,"borrow_mut","","",4,[[["self"]],["t"]]],[11,"into","","",5,[[["self"]],["u"]]],[11,"from","","",5,[[["t"]],["t"]]],[11,"try_from","","",5,[[["u"]],["result"]]],[11,"borrow","","",5,[[["self"]],["t"]]],[11,"get_type_id","","",5,[[["self"]],["typeid"]]],[11,"try_into","","",5,[[["self"]],["result"]]],[11,"borrow_mut","","",5,[[["self"]],["t"]]],[11,"as_any","","",4,[[["self"]],["any"]]],[11,"as_any","","",5,[[["self"]],["any"]]],[11,"execute","","Executes an different logic for each variant of the `enum`.",4,[[["self"],["table"]],["result",["error"]]]],[11,"execute","","Executes an different logic for each variant of the `enum`. If it succeeds, it will print `Executed.\\n` to the stdout.",5,[[["self"],["table"]],["result",["error"]]]],[11,"eq","","",4,[[["self"],["metacommand"]],["bool"]]],[11,"eq","","",5,[[["self"],["statement"]],["bool"]]],[11,"ne","","",5,[[["self"],["statement"]],["bool"]]],[11,"eq","scoolite","",2,[[["self"],["error"]],["bool"]]],[11,"ne","","",2,[[["self"],["error"]],["bool"]]],[11,"clone","","",2,[[["self"]],["error"]]],[11,"fmt","","Just writes the `message` of the `Error` to the `stdout`.",2,[[["self"],["formatter"]],["result"]]],[11,"fmt","","A row like: `Row { id: 1, username: \"john\".to_string(), email: \"john@mailbox.com\".to_string() }`",0,[[["self"],["formatter"]],["result"]]],[11,"fmt","scoolite::command","",4,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",5,[[["self"],["formatter"]],["result"]]],[11,"fmt","scoolite","",2,[[["self"],["formatter"]],["result"]]],[11,"description","","The description is just the private field `message`.",2,[[["self"]],["str"]]]],"paths":[[3,"Row"],[3,"Table"],[4,"Error"],[8,"AsAny"],[4,"MetaCommand"],[4,"Statement"],[8,"Command"]]};
initSearch(searchIndex);
