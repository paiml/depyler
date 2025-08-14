# GDB initialization script for Depyler debugging
# Source: /tmp/.tmprm1xQE/GTN_unqsn7yb7Pjs.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
