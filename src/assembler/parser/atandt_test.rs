#[cfg(test)]
mod parser_tests{
    use super::super::atandt_main::*;

    #[test]
    fn case1_test() {
        let s = "	.globl	foo
    .type	foo, @function
foo:
    pushq	%rbp
    movq	%rsp, %rbp
    movq	$30, %rax
    popq	%rbp
    ret
    .globl	main
    .type	main, @function
main:
    pushq	%rbp
    movq	%rsp, %rbp
    movq	$0, %rax
    call	foo
    popq	%rbp
    ret".to_string();

        let syms = parse_atandt(s);

        for s in syms.iter() {
            eprintln!("{}", s.0);
        }
        assert_eq!(2, syms.len());
        assert!(syms.get("foo").unwrap().is_global() && syms.get("foo").unwrap().is_function());
        assert!(syms.get("main").unwrap().is_global() && syms.get("main").unwrap().is_function());

        assert_eq!(5, syms.get("foo").unwrap().groups[0].insts.len());
        assert_eq!(6, syms.get("main").unwrap().groups[0].insts.len());
    }
}