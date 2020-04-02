#![allow(non_snake_case)]
use crate::Node::*;

/// create Beam component
/// ```
/// let beam = createBeam(nodeI, nodeJ)
/// ````
pub fn createBeam(PtIin: Pt, PtJin: Pt, grp: &mut BeamGroup) {
    let beam = Beam{ptI: PtIin, ptJ: PtJin};
    grp.addGroup(beam);
}

/// beam component has node-I & node-J
pub struct Beam{
    ptI : Pt,
    ptJ : Pt
}


pub fn createBeamGroup() -> BeamGroup{
    BeamGroup{beamGroup: Vec::new()}
}

pub struct BeamGroup{
    beamGroup: Vec< Beam > 
}

impl BeamGroup{
    /// calculate distance to other node
    /// ```
    /// use CalcArc::GroupOfBeam;
    /// let nodeThis = GroupOfBeam::createBeam(0., 0.);
    /// let nodeTarget = GroupOfBeam::createBeam(3., 4.);
    /// let dis = nodeThis.getDistanceTo(&nodeTarget);
    /// ```
    pub fn addGroup(&mut self, beam: Beam) 
    {
        self.beamGroup.push(beam);
    }

    pub fn showGroup(&self){
        for i in &self.beamGroup{
            println!("Ix:{}, Iy:{}, Jx:{}, Jy:{}",i.ptI.getX(), i.ptI.getY(), i.ptJ.getX(), i.ptJ.getY());
        }
    }

    pub fn getBeamGroup(&self) -> &Vec<Beam>{
        &self.beamGroup
    }
    //pub fn createSVG(&self){
        //let vec = self.getBeamGroup();
        //let mut vec2 :Vec<(f64, f64)> = vec![];
        //for i in vec{
            //vec2.push((i.ptI.getX() as f64, i.ptI.getY() as f64));
            //vec2.push((i.ptJ.getX() as f64, i.ptJ.getY() as f64));
        //}
        //let s = Line::new(&vec2[..]);

        //let v = ContinuousView::new()
            //.add(&s)
            //.x_range(-5., 10.)
            //.y_range(-2., 6.)
            //.x_label("x")
            //.y_label("y");

        //Page::single(&v).save("Beam.svg").unwrap();
    //}
}

//#[test]
//fn createBeam_test()
//{
    //let nodeI = createNode(0., 0.);
    //let nodeJ = createNode(100., 200.);
    //let beam = createBeam(nodeI, nodeJ);
    //assert_eq!(beam.getLength(), (100.0_f64.powf(2.) + 200.0_f64.powf(2.)).powf(0.5));
//}