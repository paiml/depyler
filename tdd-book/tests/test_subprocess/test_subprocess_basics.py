"""
TDD Book - Phase 3: Concurrency
Module: subprocess - Process execution and communication
Coverage: run(), Popen, pipes, communication, return codes

Test Categories:
- Basic process execution (run, check_output)
- Process communication (stdin, stdout, stderr)
- Popen advanced control (poll, wait, terminate, kill)
- Pipes and redirection
- Timeouts and cancellation
- Return codes and exceptions
- Shell execution
- Environment and working directory
- Edge cases and error handling
"""

import pytest
import subprocess
import sys
import os
import tempfile
import time


class TestSubprocessRun:
    """Test subprocess.run() for simple process execution."""

    def test_run_simple_command(self):
        """Property: run() executes command and returns result."""
        result = subprocess.run(["echo", "hello"], capture_output=True, text=True)

        assert result.returncode == 0
        assert "hello" in result.stdout

    def test_run_with_args(self):
        """Property: run() passes arguments to command."""
        result = subprocess.run(
            [sys.executable, "-c", "print(2 + 3)"],
            capture_output=True,
            text=True
        )

        assert result.returncode == 0
        assert "5" in result.stdout

    def test_run_capture_output(self):
        """Property: capture_output=True captures stdout/stderr."""
        result = subprocess.run(
            [sys.executable, "-c", "import sys; sys.stdout.write('out'); sys.stderr.write('err')"],
            capture_output=True,
            text=True
        )

        assert "out" in result.stdout
        assert "err" in result.stderr

    def test_run_text_mode(self):
        """Property: text=True returns str instead of bytes."""
        result = subprocess.run(["echo", "hello"], capture_output=True, text=True)

        assert isinstance(result.stdout, str)
        assert isinstance(result.stderr, str)

    def test_run_binary_mode(self):
        """Property: text=False returns bytes."""
        result = subprocess.run(["echo", "hello"], capture_output=True, text=False)

        assert isinstance(result.stdout, bytes)
        assert isinstance(result.stderr, bytes)

    def test_run_return_code(self):
        """Property: run() captures process return code."""
        result = subprocess.run([sys.executable, "-c", "exit(42)"])

        assert result.returncode == 42

    def test_run_check_success(self):
        """Property: check=True raises on non-zero exit."""
        with pytest.raises(subprocess.CalledProcessError) as exc_info:
            subprocess.run(
                [sys.executable, "-c", "exit(1)"],
                check=True
            )

        assert exc_info.value.returncode == 1

    def test_run_check_no_raise_on_success(self):
        """Property: check=True doesn't raise on success."""
        result = subprocess.run(
            [sys.executable, "-c", "exit(0)"],
            check=True
        )

        assert result.returncode == 0

    def test_run_timeout(self):
        """Property: timeout raises TimeoutExpired."""
        with pytest.raises(subprocess.TimeoutExpired):
            subprocess.run(
                [sys.executable, "-c", "import time; time.sleep(10)"],
                timeout=0.1
            )

    def test_run_timeout_success(self):
        """Property: Commands completing before timeout succeed."""
        result = subprocess.run(
            [sys.executable, "-c", "print('done')"],
            timeout=5.0,
            capture_output=True,
            text=True
        )

        assert result.returncode == 0
        assert "done" in result.stdout


class TestSubprocessPopen:
    """Test subprocess.Popen for advanced process control."""

    def test_popen_basic(self):
        """Property: Popen creates process object."""
        proc = subprocess.Popen(
            [sys.executable, "-c", "print('hello')"],
            stdout=subprocess.PIPE,
            text=True
        )
        stdout, _ = proc.communicate()

        assert proc.returncode == 0
        assert "hello" in stdout

    def test_popen_communicate(self):
        """Property: communicate() returns stdout and stderr."""
        proc = subprocess.Popen(
            [sys.executable, "-c", "import sys; print('out'); print('err', file=sys.stderr)"],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        stdout, stderr = proc.communicate()

        assert "out" in stdout
        assert "err" in stderr

    def test_popen_poll(self):
        """Property: poll() checks if process has terminated."""
        proc = subprocess.Popen([sys.executable, "-c", "import time; time.sleep(0.1)"])

        # Should be running
        assert proc.poll() is None

        proc.wait()

        # Should be terminated
        assert proc.poll() is not None

    def test_popen_wait(self):
        """Property: wait() blocks until process terminates."""
        proc = subprocess.Popen([sys.executable, "-c", "import time; time.sleep(0.01)"])
        returncode = proc.wait()

        assert returncode == 0
        assert proc.poll() is not None

    def test_popen_wait_timeout(self):
        """Property: wait() with timeout raises TimeoutExpired."""
        proc = subprocess.Popen([sys.executable, "-c", "import time; time.sleep(10)"])

        with pytest.raises(subprocess.TimeoutExpired):
            proc.wait(timeout=0.1)

        proc.kill()
        proc.wait()

    def test_popen_terminate(self):
        """Property: terminate() stops process gracefully."""
        proc = subprocess.Popen([sys.executable, "-c", "import time; time.sleep(10)"])

        proc.terminate()
        proc.wait(timeout=1.0)

        assert proc.poll() is not None

    def test_popen_kill(self):
        """Property: kill() forcefully stops process."""
        proc = subprocess.Popen([sys.executable, "-c", "import time; time.sleep(10)"])

        proc.kill()
        proc.wait(timeout=1.0)

        assert proc.poll() is not None

    def test_popen_pid(self):
        """Property: Popen has process ID."""
        proc = subprocess.Popen([sys.executable, "-c", "pass"])
        pid = proc.pid

        assert isinstance(pid, int)
        assert pid > 0

        proc.wait()

    def test_popen_stdin_input(self):
        """Property: Can write to process stdin."""
        proc = subprocess.Popen(
            [sys.executable, "-c", "data = input(); print(f'got: {data}')"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            text=True
        )
        stdout, _ = proc.communicate(input="hello\n")

        assert "got: hello" in stdout


class TestProcessCommunication:
    """Test process communication via pipes."""

    def test_pipe_stdout(self):
        """Property: PIPE captures stdout."""
        proc = subprocess.Popen(
            [sys.executable, "-c", "print('output')"],
            stdout=subprocess.PIPE,
            text=True
        )
        stdout, _ = proc.communicate()

        assert "output" in stdout

    def test_pipe_stderr(self):
        """Property: PIPE captures stderr."""
        proc = subprocess.Popen(
            [sys.executable, "-c", "import sys; print('error', file=sys.stderr)"],
            stderr=subprocess.PIPE,
            text=True
        )
        _, stderr = proc.communicate()

        assert "error" in stderr

    def test_redirect_stderr_to_stdout(self):
        """Property: stderr can redirect to stdout."""
        proc = subprocess.Popen(
            [sys.executable, "-c", "import sys; print('out'); print('err', file=sys.stderr)"],
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True
        )
        stdout, stderr = proc.communicate()

        assert "out" in stdout
        assert "err" in stdout
        assert stderr is None

    def test_devnull_suppression(self):
        """Property: DEVNULL suppresses output."""
        proc = subprocess.Popen(
            [sys.executable, "-c", "print('hidden')"],
            stdout=subprocess.DEVNULL
        )
        proc.wait()

        assert proc.returncode == 0

    def test_communicate_input(self):
        """Property: communicate() can send input."""
        proc = subprocess.Popen(
            [sys.executable, "-c", "x = input(); print(f'Echo: {x}')"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            text=True
        )
        stdout, _ = proc.communicate(input="test")

        assert "Echo: test" in stdout


class TestCheckOutput:
    """Test subprocess.check_output() convenience function."""

    def test_check_output_basic(self):
        """Property: check_output() returns stdout."""
        output = subprocess.check_output(
            [sys.executable, "-c", "print('hello')"],
            text=True
        )

        assert "hello" in output

    def test_check_output_error(self):
        """Property: check_output() raises on non-zero exit."""
        with pytest.raises(subprocess.CalledProcessError) as exc_info:
            subprocess.check_output(
                [sys.executable, "-c", "exit(1)"],
                text=True
            )

        assert exc_info.value.returncode == 1

    def test_check_output_stderr(self):
        """Property: check_output() can capture stderr."""
        output = subprocess.check_output(
            [sys.executable, "-c", "import sys; print('err', file=sys.stderr)"],
            stderr=subprocess.STDOUT,
            text=True
        )

        assert "err" in output


class TestShellExecution:
    """Test shell=True execution."""

    def test_shell_simple(self):
        """Property: shell=True executes shell commands."""
        result = subprocess.run(
            "echo hello",
            shell=True,
            capture_output=True,
            text=True
        )

        assert result.returncode == 0
        assert "hello" in result.stdout

    def test_shell_pipes(self):
        """Property: shell=True supports shell pipes."""
        result = subprocess.run(
            "echo hello | tr a-z A-Z",
            shell=True,
            capture_output=True,
            text=True
        )

        assert result.returncode == 0
        assert "HELLO" in result.stdout

    def test_shell_exit_code(self):
        """Property: shell=True captures exit codes."""
        result = subprocess.run("exit 42", shell=True)

        assert result.returncode == 42


class TestEnvironment:
    """Test environment variable handling."""

    def test_env_inherit(self):
        """Property: Process inherits environment by default."""
        os.environ["TEST_VAR"] = "test_value"
        result = subprocess.run(
            [sys.executable, "-c", "import os; print(os.environ.get('TEST_VAR', 'none'))"],
            capture_output=True,
            text=True
        )

        assert "test_value" in result.stdout
        del os.environ["TEST_VAR"]

    def test_env_override(self):
        """Property: env parameter overrides environment."""
        result = subprocess.run(
            [sys.executable, "-c", "import os; print(os.environ.get('CUSTOM', 'none'))"],
            env={"CUSTOM": "value"},
            capture_output=True,
            text=True
        )

        assert "value" in result.stdout

    def test_env_empty(self):
        """Property: Empty env clears most environment variables."""
        result = subprocess.run(
            [sys.executable, "-c", "import os; print(len(os.environ))"],
            env={},
            capture_output=True,
            text=True
        )

        # May have 1-2 system variables, but much less than normal
        count = int(result.stdout.strip())
        assert count < 5


class TestWorkingDirectory:
    """Test working directory handling."""

    def test_cwd_change(self):
        """Property: cwd parameter changes working directory."""
        with tempfile.TemporaryDirectory() as tmpdir:
            result = subprocess.run(
                [sys.executable, "-c", "import os; print(os.getcwd())"],
                cwd=tmpdir,
                capture_output=True,
                text=True
            )

            assert tmpdir in result.stdout

    def test_cwd_default(self):
        """Property: Default cwd is parent process cwd."""
        current = os.getcwd()
        result = subprocess.run(
            [sys.executable, "-c", "import os; print(os.getcwd())"],
            capture_output=True,
            text=True
        )

        assert current in result.stdout


class TestEdgeCases:
    """Test edge cases and error handling."""

    def test_command_not_found(self):
        """Property: Non-existent command raises FileNotFoundError."""
        with pytest.raises(FileNotFoundError):
            subprocess.run(["nonexistent_command_xyz"])

    def test_empty_command_list(self):
        """Property: Empty command list raises ValueError."""
        with pytest.raises((ValueError, IndexError)):
            subprocess.run([])

    def test_process_return_code_access(self):
        """Property: Can access returncode after process ends."""
        proc = subprocess.Popen([sys.executable, "-c", "exit(5)"])
        proc.wait()

        assert proc.returncode == 5

    def test_multiple_communicate_calls(self):
        """Property: Second communicate() raises ValueError."""
        proc = subprocess.Popen(
            [sys.executable, "-c", "print('hello')"],
            stdout=subprocess.PIPE,
            text=True
        )
        stdout1, _ = proc.communicate()

        assert "hello" in stdout1

        # Second communicate() should raise ValueError (pipes closed)
        with pytest.raises(ValueError):
            proc.communicate()

    def test_large_output(self):
        """Property: Can handle large output."""
        result = subprocess.run(
            [sys.executable, "-c", "print('x' * 100000)"],
            capture_output=True,
            text=True
        )

        assert len(result.stdout) > 100000

    def test_context_manager_popen(self):
        """Property: Popen works as context manager."""
        with subprocess.Popen(
            [sys.executable, "-c", "print('hello')"],
            stdout=subprocess.PIPE,
            text=True
        ) as proc:
            stdout, _ = proc.communicate()

        assert "hello" in stdout
        assert proc.returncode == 0
