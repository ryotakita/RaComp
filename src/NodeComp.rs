#![allow(non_snake_case)]
use std::collections::HashMap;
/// ```
/// use CalcArc::GroupOfNode;
/// let nodeThis = GroupOfNode::createNode(0., 0.);
/// let nodeTar = GroupOfNode::createNode(3., 4.);
/// let Dis = GroupOfNode::calcDistance(&nodeThis, &nodeTar);
/// ````
pub fn calcDistance(node1: &Node, node2: &Node) -> f64
{
    let disX = (node1.pt.x - node2.pt.x).abs();
    let disY = (node1.pt.y - node2.pt.y).abs();
    let tmp = (disX.pow(2) + disY.pow(2)) as f64;
    tmp.powf(0.5)
}

/// createNode
/// ```
/// use CalcArc::GroupOfNode;
/// let node = GroupOfNode::createNode(0., 0.);
/// ```
pub fn createNode(xtmp: i64, ytmp: i64, nodeGroup: &mut NodeGroup)
{
    let pt = Pt{x:xtmp, y:ytmp};
    let node = Node{pt};
    nodeGroup.addGroup(node);
}

pub fn createPt(xtmp: i64, ytmp: i64) -> Pt{
    Pt{x: xtmp, y: ytmp}
}

// Pt has X & Y
#[derive(PartialEq, Eq, Hash,Clone, Copy)]
pub struct Pt{
    x: i64,
    y: i64
}

impl Pt{

    pub fn getX(&self) -> i64{
        self.x
    }

    pub fn getY(&self) -> i64{
        self.y
    }
}

/// Node has X & Y
pub struct Node {
    pt: Pt
}

impl Node{

    /// 節点の座標を取得します。
    fn getPt(&self) -> Pt{
        self.pt
    }
    /// 自身から、対象の節点との距離を取得します。
    /// ## Arguments
    /// * `nodeTar` - 対象節点:Node
    pub fn getDistanceTo(&self, nodeTar: &Node) -> f64
    {
        let temp = ((self.pt.x - nodeTar.pt.x).abs().pow(2) + (self.pt.y - nodeTar.pt.y).abs().pow(2)) as f64;
        temp.powf(0.5)
    }

    /// X座標を取得します。
    pub fn getX(&self) -> i64
    {
        self.pt.x
    }

    /// Y座標を取得します。
    pub fn getY(&self) -> i64
    {
        self.pt.y
    }
}

/// 節点の管理クラスです。
/// TODOk:シングルトンで実装？
pub struct NodeGroup{
    nodeGroup: HashMap<Pt, Node>
}

/// 節点の管理クラスを生成します。
/// ## Assert
/// - 通常、解析開始時の一回しか呼び出されません。
pub fn createNodeGroup() -> NodeGroup{
    NodeGroup{nodeGroup: HashMap::new()}
}

impl NodeGroup{

    /// 節点管理クラスに節点を追加します。
    /// ## Assert
    /// - 節点生成時に自動的に呼び出されます。
    pub fn addGroup(&mut self, node: Node) 
    {
        self.nodeGroup.entry(node.getPt()).or_insert(node);
    }

    /// 節点管理クラスに存在する節点の座標を返します。
    /// 現時点ではテスト用です。
    /// ## Assert
    /// - 本当はNodeをCopyTraitで返したいけど難しい...
    pub fn findGroup(self, x: i64, y: i64) -> Pt
    {
        let pt = createPt(x, y);
        let node = self.nodeGroup.get(&pt);
        match node{
            Some(n) => n.getPt(),
            None => createPt(0, 0)
        }
    }

    /// 節点の一覧を出力します。
    /// ## Assert
    /// - 現在は直接プリントしていますが、将来的にはリストを渡します。
    pub fn showGroup(&self){
        for (k,v) in &self.nodeGroup{
            println!("x:{}, y:{}",k.x, k.y);
        }
    }

    //pub fn findNodeByPt(&self, x: f64, y:f64) -> &Node{
    //    self.nodeGroup.iter().find(|&node| node.getX() == x && node.getY() == y).unwrap()
    //}

    //pub fn getNodeGroup(&self) -> &Vec<Node>{
        //&self.nodeGroup
    //}
    //pub fn createSVG(&self){
        //let vec = self.getNodeGroup();
        //let mut vec2 :Vec<(f64, f64)> = vec![];
        //for i in vec{
            //vec2.push((i.getX(), i.getY()));
        //}
        //let s = Scatter::from_slice(&vec2).style(
            //Style::new().colour("#35C788"),
        //);

        //let v = ContinuousView::new()
            //.add(&s)
            //.x_range(-5., 10.)
            //.y_range(-2., 6.)
            //.x_label("x")
            //.y_label("y");

        //Page::single(&v).save("Node.svg").unwrap();
    //}

}


#[test]
fn structNode_test()
{
    let mut nodeGroup = createNodeGroup();
    let node = createNode(100, 200, &mut nodeGroup);

    let pt = nodeGroup.findGroup(100, 200);
    assert_eq!(pt.getX(), 100);
    assert_eq!(pt.getY(), 200);
}

//TODOk::一旦Beamもここで定義するが、これは本来はBeamCompにあるべき関数群
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
