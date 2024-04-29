# Save ONNX
## use  postgresql
### DONE with  [pg_onnx](https://github.com/kibae/pg_onnx)
1  No CMAKE_CXX_COMPILER could be found.  -> sudo apt-get install -y build-essential
2  Could NOT find PkgConfig (missing: PKG_CONFIG_EXECUTABLE) -> sudo apt-get install pkg-config
3  This SQL extenation can save ONNX file in postgresql,  and offer us its reference for querying, but no API for reading ONNX back.

### DONE with postgresql bytes attribute
- save ONNX file with bytea attribute
  #+begin_src sql
    CREATE TABLE onnxsets (
          id SERIAL PRIMARY KEY,
          onnxfile bytea
    );
  #+end_src
- limited to 1 GB, enough for our onnx file, be careful about the permission and ownership of ONNX file for postgresql.
- save file with pg_read_binary_file, and read it out with copy, but with exra header and footer information, which increase the size about 30 bytes, [postgresql copy manual](https://www.postgresql.org/docs/current/sql-copy.html).
  ```sql
  insert into onnxsets (onnxfile ) values (pg_read_binary_file('/home/postgres/resnet50_Opset16.onnx')::bytea);
  \copy (select onnxfile from onnxsets where id = 8) to '/home/postgres/testonnx.onnx' (FORMAT binary) ;
  ```

- Those information destroy the readability of ONNX file, such as in [https://netron.app/](https://netron.app/) we get the error for reading returned ONNX file from postgresql.
   ``` 
   Error loading model. Unsupported file content (5047434f50590aff0d0a000000000000) for extension '.onnx'.
   ```
- Tried  [pgsql-fio extenation](https://github.com/csimsek/pgsql-fio/), still does not work.  Up to now, I still do not find ang way to get rid of those header and footer information.
  
### TODO with postgresql BLOB  attribute

## TODO use MongoDB
## TODO use localhost file system
If we do not benefit enough for performance with saving ONNX in database, we can save ONNX file in localhost file system with a unique name, and then save this path + unique name in postgresql as string.

