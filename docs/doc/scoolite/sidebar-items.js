initSidebarItems({"enum":[["Error","Main `Error` type, each `enum` variant represents a different error."]],"fn":[["build_command","This function is just a proxy that creates a `Command` or returns an `Error`. The way it decides if it will return a `MetaCommand` or a `Statement` is by looking on the `String` `input` if it starts with a dot (`.`)."],["print_error","Just prints an `Error`'s `description` to the terminal."],["print_prompt","Just prints `db >` to the terminal."],["read_input","This function flushes the stdout, so that no outputs to the terminal get's stuck. Then it reads a line from stdin and returns it as a `String`."]],"mod":[["as_any","Module for `AsAny` trait"],["command","Module for `Command`s/`Statement`s of the SQL-like language"],["error","Module for `Error` type"],["io","Module for interacting with stdio and stdout"],["row","Module for `Row` type"],["table","Module for `Table` type"]],"struct":[["Row","This struct contains the first table available for storage. It will be defined by the user eventually."],["Table","In memory storage of `Row`s."]],"trait":[["AsAny","This trait has the purpose of casting between trait objects and Sized structures/enums."],["Command","The interface that every `Command` asks for is just an `execute` method, which executes the specific logic for the `Command`."]]});