:- module(dif, [dif/2]).

:- use_module(library(atts)).
:- use_module(library(control), [(\=)/2]).

:- attribute dif/2.

dif_set_variables([], _, _).
dif_set_variables([Var|Vars], X, Y) :-
    put_atts(Var, dif(X, Y)),
    dif_set_variables(Vars, X, Y).

verify_dif_attrs([dif(X, Y) | Attrs], Var, [X \== Y | Goals]) :-
    (   get_atts(Var, +dif(X, Y)) -> true
    ;   put_atts(Var, +dif(X, Y))
    ),
    verify_dif_attrs(Attrs, Var, Goals).
verify_dif_attrs([_ | Attrs], Var, Goals) :-
    verify_dif_attrs(Attrs, Var, Goals).
verify_dif_attrs([], _, []).

verify_dif_attrs_no_var([dif(X, Y) | Attrs], [X \== Y | Goals]) :-
    verify_dif_attrs_no_var(Attrs, Goals).
verify_dif_attrs_no_var([_ | Attrs], Goals) :-
    verify_dif_attrs_no_var(Attrs, Goals).
verify_dif_attrs_no_var([], []).

verify_attributes(Var, Value, Goals) :-
    (   get_atts(Var, Attrs) ->
	(   var(Value) -> verify_dif_attrs(Attrs, Value, Goals)
	;   verify_dif_attrs_no_var(Attrs, Goals)
	)
    ).

% Probably the world's worst dif/2 implementation. I'm open to
% suggestions for improvement.

dif(X, Y) :- X \== Y,
             (   X \= Y -> true
             ;   term_variables(X, XVars), term_variables(Y, YVars),
	         dif_set_variables(XVars, X, Y),
		 dif_set_variables(YVars, X, Y)
	     ).

gather_dif_goals(Attrs, _) :-
    var(Attrs), !.
gather_dif_goals([dif(X, Y) | Attrs], [dif(X, Y) | Goals]) :-
    gather_dif_goals(Attrs, Goals).
gather_dif_goals([_ | Attrs], Goals) :-
    gather_dif_goals(Attrs, Goals).

attribute_goals(X, Goal) :-
    '$get_attr_list'(X, Attrs),
    gather_dif_goals(Attrs, Goal).