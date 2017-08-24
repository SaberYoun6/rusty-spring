 //use branchPrediction::*;
// use streflop_cond::*;
// use rustreg_cond::*;
// use fastmath::*;
 //use win64::*
 extern crate core;
 extern crate clamp2;
 use clamp::*;
 use std::env::*;
 use core::f64::consts;
 use std::default::*;
 use std::ops::DivAssign;
 use std::ops::MulAssign;
 use std::ops::AddAssign;
 use std::ops::SubAssign;
 use core::cmp::PartialOrd;
 use core::iter::Iterator;
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
struct intFloat3{
    t: i64 ,
}
impl intFloat3 {
    fn origin() -> intFloat3 {
        intFloat3 { t : 0 }
    }
    fn new ( t : i64 ) -> intFloat3 {
        intFloat3 { t : t}
    }
}
struct floatMaxXPos{
   maxxpos : f64,
}
impl floatMaxXPos {
    fn defer_maxxpos() -> floatMaxXPos {
        floatMaxXPos { maxxpos : -1.0 }
    }
    fn new_maxxpos(maxxpos : f64)-> floatMaxXPos{
        floatMaxXPos { maxxpos : maxxpos }
    }
}
struct floatMaxYPos {
    maxypos : f64,
}
impl floatMaxYPos {
    fn defer_maxypos() -> floatMaxYPos {
        floatMaxYPos { maxypos : -1.0}
    }
    fn new_maxypos(maxypos : f64) ->floatMaxYPos{
        floatMaxYPos{ maxypos : maxypos}
    }
}
struct floati3
{

}

impl floatI3 {
    fn is_in_bounds(  maxxpos : f64 ,  maxypos : f64,  x : f64 , y :f64) -> bool 
    {
        let minx = 0.0;
        assert!(maxxpos > minx, "maxx= {:?},minx = {:?}",maxxpos,minx);
        return(x >= 0.0 && x <= maxxpos  ) && ( y >= 0.0 && y <= maxypos);
    }
    fn is_in_map( maxxpos : f64, maxypos: f64 , x:f64, y:f64) -> bool
    {
        let minx = 0.0;
        assert!(maxxpos > minx , "maxxpos = {:?}, minx = {:?}", maxxpos, minx);
         return(x >= 0.0 && x <= maxxpos) && ( y >= 0.0 && y <=maxypos);
    }
    fn clamp_in_bounds(maxxpos : f64, maxypos : f64, x : f64 ,y : f64) -> ()
    {
        let minx = 0.0;
        assert!(maxxpos > minx, "maxxpos= {:?}, minx = {:?}", maxxpos, minx);

        let x=clamp2::clamp(x,minx,maxxpos);
        let y=clamp2::clamp(y,minx,maxypos);
    }
}


fn main(){
    let maxxpos =   0.2;
    let maxypos =   0.1;
    let x = 1.0987;
    let y = 2.9080;
    let z = 8.9123;
    let wxz=float3{x:x,y:y,z:z};
     println!("{:?} i just want to see the result ", floatI3::clamp_in_bounds(maxxpos,maxypos,x,z));
    if floatI3::is_in_bounds(maxxpos, maxypos, x, y) {
        println!("then this function is working");
    } else {
       println!("there is no results");
    }
    if floatI3::is_in_map(maxypos , maxypos, x ,y) {
        println!("I'm going to assume that this is also going to panic");
        }
        else {
        println!(" nothing" );
        }
        
}
