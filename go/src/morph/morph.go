// package morph defines and implements a simple state rewrite unit,
// called a "Morph".  A Morph is a rewriting ruleset which operates on
// some state.  The result may be written over the original state, or it
// may be written to another register.
//
// For example:
//
// > Rev: Reverse a sequence
// >
// Rev {|m| }

package morph

import "reflect"

// Morph is a map of permitted transforms over some value.
type Morph struct {
	reflect.Kind
}
