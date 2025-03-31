fn main() {
    println!(
        "use std::alloc::Layout;

use crate::{{Field, FieldFlags, Innards, Scalar, Shape, ShapeDesc, Shapely, mini_typeid}};

impl Shapely for () {{
    fn shape() -> Shape {{
        Shape {{
            name: |f| write!(f, \"()\"),
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<()>(),
            innards: Innards::Scalar(Scalar::Nothing),
            set_to_default: Some(|_addr: *mut u8| {{}}),
            drop_in_place: None,
        }}
    }}
}}

impl<T1: Shapely> Shapely for (T1,)
where
    T1: Shapely,
{{
    fn shape() -> Shape {{
        struct FieldsMaker<T1> {{
            _phantom: std::marker::PhantomData<T1>,
        }}

        impl<T1> FieldsMaker<T1>
        where
            T1: Shapely,
        {{
            const FIELDS: [Field; 1] = [Field {{
                name: \"0\",
                shape: ShapeDesc(T1::shape),
                offset: 0,
                flags: FieldFlags::EMPTY,
            }}];
        }}

        Shape {{
            name: |f| {{
                write!(f, \"(\")?;
                (T1::shape().name)(f)?;
                write!(f, \",)\")
            }},
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<(T1,)>(),
            innards: Innards::Struct {{
                fields: &FieldsMaker::<T1>::FIELDS,
            }},
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {{
                std::ptr::drop_in_place(addr as *mut (T1,));
            }}),
        }}
    }}
}}"
    );

    // Generate implementations for tuples of size 2 to 12
    for n in 2..=12 {
        let type_params = (1..=n)
            .map(|i| format!("T{}", i))
            .collect::<Vec<_>>()
            .join(", ");

        let fields = (0..n)
            .map(|i| {
                format!(
                    "Field {{
                name: \"{}\",
                shape: ShapeDesc(T{}::shape),
                offset: std::mem::offset_of!(({},), {}),
                flags: FieldFlags::EMPTY,
            }}",
                    i,
                    i + 1,
                    type_params,
                    i
                )
            })
            .collect::<Vec<_>>()
            .join(", ");

        let mut name_format = "write!(f, \"(\")?;".to_string();
        for i in 1..=n {
            name_format += &format!("\n                (T{}::shape().name)(f)?;", i);
            if i < n {
                name_format += "\n                write!(f, \",\")?;";
            }
        }
        name_format += "\n                write!(f, \",)\")";

        let where_clause = (1..=n)
            .map(|i| format!("T{}: Shapely", i))
            .collect::<Vec<_>>()
            .join(", ");

        println!(
            "
impl<{type_params}> Shapely for ({type_params},)
where
    {where_clause},
{{
    fn shape() -> Shape {{
        struct FieldsMaker<{type_params}> {{
            _phantom: std::marker::PhantomData<({type_params},)>,
        }}

        impl<{type_params}> FieldsMaker<{type_params}>
        where
            {where_clause},
        {{
            const FIELDS: [Field; {n}] = [{fields}];
        }}

        Shape {{
            name: |f| {{
                {name_format}
            }},
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<({type_params},)>(),
            innards: Innards::Struct {{
                fields: &FieldsMaker::<{type_params}>::FIELDS,
            }},
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {{
                std::ptr::drop_in_place(addr as *mut ({type_params},));
            }}),
        }}
    }}
}}"
        );
    }
}
