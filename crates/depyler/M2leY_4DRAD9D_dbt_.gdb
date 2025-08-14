# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpGwcg6d/M2leY_4DRAD9D_dbt_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
