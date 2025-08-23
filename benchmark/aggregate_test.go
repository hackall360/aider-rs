package main

import "testing"

func TestAggregate(t *testing.T) {
	cases := []CaseResult{{Name: "a", Passed: true}, {Name: "b", Passed: false}}
	s := aggregate(cases)
	if s.Total != 2 || s.Passed != 1 || s.Failed != 1 {
		t.Fatalf("unexpected stats: %+v", s)
	}
}
