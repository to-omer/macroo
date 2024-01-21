use super::ast::*;
use quote::{quote, ToTokens, TokenStreamExt};

impl ToTokens for Ast {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(quote!{
            #[allow(unused)] let mut stack: ::std::vec::Vec<i64> = ::std::default::Default::default();
            #[allow(unused)] let mut heap: ::std::collections::HashMap<i64, i64> = ::std::default::Default::default();
        });
        tokens.append_all(&self.commands);
    }
}

impl ToTokens for Command {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Command::Stack(command) => command.to_tokens(tokens),
            Command::Heap(command) => command.to_tokens(tokens),
            Command::Arith(command) => command.to_tokens(tokens),
            Command::Flow(command) => command.to_tokens(tokens),
            Command::Io(command) => command.to_tokens(tokens),
        }
    }
}

impl ToTokens for Number {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl ToTokens for Stack {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(match self {
            Stack::Push(number) => quote! {
                stack.push(#number);
            },
            Stack::Duplicate => quote! {
                stack.push(stack.last().unwrap().clone());
            },
            Stack::Swap => quote! {{
                let len = stack.len();
                stack.swap(len - 1, len - 2);
            }},
            Stack::Discard => quote! {
                stack.pop().unwrap();
            },
            Stack::Copy(number) => quote! {
                stack.push(stack[stack.len() - #number - 1]);
            },
            Stack::Slide(number) => quote! {{
                let value = stack.pop().unwrap();
                for _ in 0..#number - 1 {
                    stack.pop().unwrap();
                }
                stack.push(value);
            }},
        });
    }
}

impl ToTokens for Heap {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(match self {
            Heap::Store => quote! {{
                let value = stack.pop().unwrap();
                heap.insert(stack.pop().unwrap(), value);
            }},
            Heap::Retrieve => quote! {{
                let address = stack.pop().unwrap();
                stack.push(heap.get(&address).cloned().unwrap());
            }},
        });
    }
}

impl ToTokens for Arith {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(match self {
            Arith::Add => quote! {{
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs + rhs);
            }},
            Arith::Sub => quote! {{
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs - rhs);
            }},
            Arith::Mul => quote! {{
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs * rhs);
            }},
            Arith::Div => quote! {{
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs / rhs);
            }},
            Arith::Mod => quote! {{
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs % rhs);
            }},
        });
    }
}

impl ToTokens for Label {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut label = String::new();
        label.push('W');
        for &bit in &self.0 {
            label.push(if bit { '1' } else { '0' });
        }
        quote!(#label).to_tokens(tokens);
    }
}

impl ToTokens for Flow {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(match self {
            Flow::Mark(label) => quote! {
                #[allow(named_asm_labels)]
                ::core::arch::asm!(::std::concat!(#label, ":"));
            },
            Flow::Call(label) => quote! {
                ::core::arch::asm!(::std::concat!("call ", #label));
            },
            Flow::Jump(label) => quote! {
                ::core::arch::asm!(::std::concat!("jmp ", #label));
            },
            Flow::JumpIfZero(label) => quote! {
                if stack.pop().unwrap() == 0 {
                    ::core::arch::asm!(::std::concat!("jmp ", #label));
                }
            },
            Flow::JumpIfNeg(label) => quote! {
                if stack.pop().unwrap() < 0 {
                    ::core::arch::asm!(::std::concat!("jmp ", #label));
                }
            },
            Flow::Return => quote! {
                ::core::arch::asm!("ret");
            },
            Flow::Exit => quote! {
                ::std::process::exit(0);
            },
        });
    }
}

impl ToTokens for Io {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(match self {
            Io::OutputChar => quote! {
                ::std::print!("{}", stack.pop().unwrap() as u8 as char);
            },
            Io::OutputNum => quote! {
                ::std::print!("{}", stack.pop().unwrap());
            },
            Io::ReadChar => quote! {{
                use ::std::io::Read;
                let mut buffer = [0u8; 1];
                ::std::io::stdin().read_exact(&mut buffer).unwrap();
                let address = stack.pop().unwrap();
                heap.insert(address, buffer[0] as i64);
            }},
            Io::ReadNum => quote! {{
                let mut buffer = ::std::string::String::new();
                ::std::io::stdin().read_line(&mut buffer).unwrap();
                let address = stack.pop().unwrap();
                heap.insert(address, buffer.trim().parse().unwrap());
            }},
        });
    }
}
