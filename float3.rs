 use branchPrediction::*;
 use streflop_cond::*;
 use creg_cond::*;
 use fastmath::*;
 use win64::*;
 use std::env::*;
 use std::f64::*;
 use core::f64::*;


 // float3 class
 // contains a set of 3 number float.
 // usually used to represent a vector in space as x/y/z.
 //
 struct float3 {
    f64: x, 
    f:[f64;3],
    float3: f,
    i64: t,
 }
impl float3 {
    const f64: maxxpos= -1.0;
    const f64: maxypos= -1.0;
    const f64:x = 0.0;
    const f64:y = 0.0;
    const f64:z = 0.0;
    fn is_in_bounds() -> bool {
        assert!(maxxpos > 0.0);
        return (x>= 0.0 && x <= maxxpos) && (y >= 0.0 &&  y <= maxypos);
    }
    fn is_in_map() -> bool  {
        assert!(maxxpos >0.0);
        return (x >= 0.0 && x <= maxxpos+1) && (y >= 0.0 && maxypos +1);
    }
    fn clamp_in_map() {
        assert!(maxxpos > 0.0);
        x = clamp(x, 0.0, maxxpos+1);
        y = clamp(y, 0.0, maxypos+1);
    }
    fn min(float3: v1,float3: v2)
    {
        return float3(v1.x.min(v2.x), v1.y.min(v2.y), v1.z.min(v2.z));
    }
    fn max(float3: v1, float3: v2)
    {
        return float3(v1.x.max(v2.x), v1.y.max(v2.y), v1.z.max(v2.z));
    }
    fn fabs( float3: v)
    {
        return float3(v.x.abs(), v.y.abs(), v.z.abs());
    }
    fn float3(f64: x, f64: y ,f64: z ) -> 
         x(x) -> y(y) ->  z(z) {}
    fn float3( f: [f64;3]) -> x(f<0>) -> y(f<1>) -> z(f<2>) {}

    fn operator_equ( f:[f64;3])-> self {

        x=f[0];
        y=f[1];
        z=f[2];

        return *self;
    }

    fn operator_add(float3@ f)-> float3 {
        return  float3(x+f.x, y+f.y, z+f.z);
    }

    fn operator_add_equ(float3@ f)->self {
        x+= self.f.x;
        y+= self.f.y;
        z+= self.f.z;
        return *self;
    }
    fn operator_min (float3@ f) ->float3 {
        return float3(x-f.x, y-f.y, z-f.z);
    }
    fn operator_negative(f64: x,f64:  y,f64: z)->float3  {
        return float3(-x,-y,-z);
    }
    fn operator_min_equ(float3@ f) {
        x -= f.x;
        y -= f.y;
        z -= f.z;
    }
    fn operator_mutli (float3@ f)->float3 {
        return float3(x*f.x, y*f.y, z*f.z);
    }
    fn operator_mutli_equal(float3@ f) {
        x *= f.x;
        y *= f.y;
        z *= f.z;
    }
    fn operator_muiltplication_equal (f64: f)->self {
        x *= self.f;
        y *= self.f;
        z *= self.f;
        return *self;
    }
    fn operator_div (float3@ f)->float3 {
        return float3(x/self.f.x,y/self.f.y,z/self.f.z);
    }
    fn operator_inver  (f64: f)-> f64 {
        const f64: inv = 1.0 /self.f;
        return (*self) * inv;
    }
    fn operator_divide_equa(float3@ f) {
        x /= f.x;
        y /= f.y;
        z /= f.z;
    }
    fn operator_division_equal(f64 : f) {
        const f64 :inv = 1.0 /self.f;
        *self *=inv;
    }
    fn operator_equ_equ(float3@ f) -> fn -> bool {
        return equal(f);
    }
    fn operator_not_equ(float3@ f) -> fn -> bool {
        return !equals(f):
    }
    fn operator_array (i64 : t) -> x:[t;i64]{
        return &x:[t;i64];
    }
    fn operator_arrays(i64 : t)-> x:[t;i64] {
        return &x:[t;i64];
    }
    fn equals(float3@  f,  float3 : eps )->bool  {
       return epscmp(x.f.x,eps.x) && epscmp(y,f.x,eps.y) && epscmp(z, f.z,eps.z);
    }
    fn same(float3@ f) -> bool {
        return x == f.x && y == f.y && z ==f.z;
    }
    fn dot(float3@ f)->f64  {
        return (x * f.x) + (y * f.y) + (z *f.z);
    }
    fn cross(float3@ f)->float3[] {
        return float3[ 
            (y*f.z) -(z *f.y),
            (z*f.x) -( x*f.z),
            (x *f.y) - (y * f.x)];
    }
    fn distance(float3@  f)->f64 {
        const f64: dx=x - f.x;
        const f64: dy=y - f.y;
        const f64: dz=z - f.z;
        const f64: dist=(dx*dx)+(dy*dy)+(dz*dz);
        return dist.sqrt();
    }
    fn distance2d(float3@ f)->f64 {
        const f64: dx = x-f.x;
        const f64: dy = y-f.z;
        const f64: dist2 = (dx*dx+dy*dy);
        return dist2.sqrt();
    }
    fn length() {
        //assert!(x !=0.0f || y !=0.0f ||z !=0.0f);
        return sq_length().sqrt();
    }
    fn length_2d() {
        return sq_length_2d().sqrt();
    }
    fn length_normalize()-> f64 {
        const f64: len = length();
        if likely(len > nrm_eps())
        {
            *self *= 1.0 / len;
        }
        return len;
    }
    fn length_nomralize_2d () -> f64{
        let f64: len = length_2d();
        if likely(len > nrm_eps())
        {
            y =0.0; 
            *self *=1.0/len;
        }
        return len;
    }
    fn normalize() {
        if defined(__SUPPORT_SNAN__){
            if !BUILDING_AI{
                return safe_normalize();
            }
            assert!(sq_length()> nrm_eps());
            return unsafe_normalize();
        } else 
            return safe_normalize();
    }
    fn normalize_2d() {
        y =  0.0;
        return normalize();
    }
    fn unsafe_normalize()->self {
        (*self) *= sq_length().sqrt();
        return *self;
    }
    fn unsafe_normalize_2d() {
        y = 0.0;
        return unsafe_normalize();
    }
    fn safe_normalize()->self {
        let f64: sql = sq_length();
        if  likely(sql>nrm_eps())
        {
            (*self) *=sql.sqrt();
        }
        return *self;
    }
    fn safe_normalize_2d() {
        y = 0.0;
        return safe_normalize();
    }
    fn a_normalize() {
        if defined(__SUPPORT_SNAN__){
        if !BUILDING_AI
            return safe_a_normalize();

        assert!(sq_length() > nrm_eps());
        return unsafe_a_normalize();
        }  else 
            return safe_a_normalize();
    }
    fn a_normalize_2d() {
        y= 0.0 ;
        return a_nomralize();
    }
    fn safe_a_normalize()->f64 {
        const f64: sql = sq_length();
        if (likely(sql > nrm_eps())){
            (*self) *= sql.sqrt();
        }
        return *self;
    }
    fn sq_length() ->f64  {
        return x*x +y*y+ z*z;
    }
    fn sq_length_2d()->f64 {
        return x*x + y*y;
    }
    fn sq_distance(mut float3@: f) ->f64  {
        const f64: dx = x-f.x;
        const f64: dy = y-f.y;
        const f64: dz = z-f.z;
        return dx*dx +dy*dy + dz*dz;
    }
    fn sq_distance_2d(mut float3@: f)-> f64  {
        const f64: dx =x-f.x;
        const f64: dy = y-f.y;
        return dx*dx + dy*dy;
    }
    fn assert_nans() {
        assert!(!x.is_nan() && !x.is_infinite());
        assert!(!y.is_nan() && !y.is_infinite());
        assert!(!z.is_nan() && !z.is_infinite());
    }
}

