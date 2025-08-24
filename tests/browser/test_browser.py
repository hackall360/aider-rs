import os
import subprocess
import unittest


class TestBrowser(unittest.TestCase):
    def test_browser_flag_launches_gui(self):
        env = os.environ.copy()
        env["AIDER_TEST_GUI"] = "1"
        result = subprocess.run(
            [
                "cargo",
                "run",
                "--quiet",
                "-p",
                "aider-cli",
                "--",
                "--browser",
                "--yes",
            ],
            capture_output=True,
            text=True,
            env=env,
        )
        self.assertEqual(result.returncode, 0)
        self.assertIn("launch_gui_called", result.stdout)


if __name__ == "__main__":
    unittest.main()
