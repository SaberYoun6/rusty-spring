 //use branchPrediction::*;
// use streflop_cond::*;
// use rustreg_cond::*;
// use fastmath::*;
 //use win64::*
 extern crate core;
 use std::env::*;
 use core::f64::consts;
 use std::default::*;
 use std::ops::DivAssign;
 use std::ops::MulAssign;
 use std::ops::AddAssign;
 use std::ops::SubAssign;
 // float3 class
 // contains a set of 3 number float.
 // usually used to represent a vector in space as x/y/z.
 //
struct float3 {
    x : f64,
    y : f64,
    z : f64,
 }
impl float3{
    fn origin() -> float3 {
        float3 { x : 0.0, y: 0.0, z : 0.0 }
    }
    fn new(x : f64 ,y : f64 , z : f64) -> float3 {
        float3 {x : x, y : y, z : z }
    }
}
 /*       
struct floater {
    f : f64,
}
impl floater {
    fn origin() -> f64{
        floater { f : 0.0 }
    }
    fn new ( f : f64)-> f64 {
        floater {f : f }
    }
}
struct float1{
    fo:[f64;3],
}
impl float1 {
  fn origin() -> float1 {
        float1 ();
  }
  fn new (fo:[f64;3]) ->float1 {
      float1 ( fo : flo);
  }     
}
struct float2{
    fl : float3,
}
impl float2 {
    fn origin() -> float2 {
        float2 ();
    }
    fn new ( fl : float3) {
        float2 ( fl : flo );
    } 
}

struct vector {
    vectors : float3,
}
impl vector {
   fn  origin_vector() -> float3 {
       vector { vectors :0.0}
   }
   fn new_vector(vectors : float3) {
       vector{ vectors : vectors}
   }
}
*/
struct int_float3{
    t: i64 ,
}
impl int_float3 {
    fn origin() -> int_float3 {
        int_float3 { t : 0 }
    }
    fn new ( t : i64 ) -> int_float3 {
        int_float3 { t : t}
    }
}
struct float_maxxpos{
   maxxpos : f64,
}
impl float_maxxpos {
    fn origin_maxxpos() -> float_maxxpos {
        float_maxxpos { maxxpos : -1.0 }
    }
    fn new_maxxpos(maxxpos : f64)-> float_maxxpos{
        float_maxxpos { maxxpos : maxxpos }
    }
}
struct float_maxypos {
    maxypos : f64,
}
impl float_maxypos {
    fn origin_maxypos() -> float_maxypos {
        float_maxypos { maxypos : -1.0}
    }
    fn new_maxypos(maxypos : f64) ->float_maxypos{
        float_maxypos{ maxypos : maxypos}
    }
}
struct floati3
{

}

impl floati3 {
    fn is_in_bounds( float_maxxpos : maxxpos  , float_maxypos : maxypos,  x : f64 , y :f64) -> bool 
    {
        let minx = 0.0;
            assert!(maxxpos > minx, "maxx= {},minx = {}",maxxpos,xminx);
        return(x >= 0.0 && x <= maxxpos  ) && ( y >= 0.0 && y <= maxypos);
    }
}


fn main(){
    let _float_maxxpos = float_maxxpos{maxxpos : -1.2};
    let _float_maxypos = float_maxypos{maxypos :1.0};
    let x = 1.0987;
    let y = 2.9080;
    let z = 8.9123;
    let wxz=float3{x:x,y:y,z:z};
    if floati3::is_in_bounds(_float_maxxpos, _float_maxypos, x, y) {
        println!("then this function is working");
    } else {
       println!("there is no results");
    }
}
