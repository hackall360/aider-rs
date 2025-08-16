package main

import (
	"fmt"
	"log"
	"os"
	"os/exec"
	"runtime"
)

func run(cmd string, args ...string) {
	c := exec.Command(cmd, args...)
	c.Stdout = os.Stdout
	c.Stderr = os.Stderr
	if err := c.Run(); err != nil {
		log.Fatalf("command failed: %s %v: %v", cmd, args, err)
	}
}

func installLinux() {
	fmt.Println("Installing Rust...")
	run("sh", "-c", "curl https://sh.rustup.rs -sSf | sh -s -- -y")

	fmt.Println("Installing Go...")
	run("sh", "-c", "curl -L https://go.dev/dl/go1.22.4.linux-amd64.tar.gz -o /tmp/go.tar.gz && sudo rm -rf /usr/local/go && sudo tar -C /usr/local -xzf /tmp/go.tar.gz")

	fmt.Println("Installing Dart...")
	run("sh", "-c", "curl -L https://storage.googleapis.com/dart-archive/channels/stable/release/latest/sdk/dartsdk-linux-x64-release.zip -o /tmp/dart.zip && sudo unzip -qo /tmp/dart.zip -d /usr/local && sudo mv /usr/local/dart-sdk /usr/local/dart")
}

func installWindows() {
	fmt.Println("Installing Rust, Go and Dart with winget...")
	run("cmd", "/C", "winget install --id Rustlang.Rustup -e --silent")
	run("cmd", "/C", "winget install --id GoLang.Go -e --silent")
	run("cmd", "/C", "winget install --id Google.Dart -e --silent")
}

func main() {
	switch runtime.GOOS {
	case "linux":
		installLinux()
	case "windows":
		installWindows()
	default:
		fmt.Println("Unsupported OS:", runtime.GOOS)
	}
}
