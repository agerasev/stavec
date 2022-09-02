use crate::StaticVec;

#[test]
fn empty() {
    let mut v = StaticVec::<i32, 4>::new();

    assert_eq!(v.len(), 0);
    assert!(v.is_empty());
    assert!(!v.is_full());

    assert_eq!(v.as_slice().len(), 0);
    assert_eq!(v.as_mut_slice().len(), 0);
}

#[test]
fn full() {
    let mut v = StaticVec::<i32, 4>::from_array([0, 1, 2, 3]);

    assert_eq!(v.len(), 4);
    assert!(!v.is_empty());
    assert!(v.is_full());
    for i in 0..v.len() {
        assert_eq!(v[i], i as i32);
    }
    for (i, x) in v.iter().enumerate() {
        assert_eq!(*x, i as i32);
    }

    for (i, x) in v.as_slice().iter().enumerate() {
        assert_eq!(*x, i as i32);
    }
    for (i, x) in v.as_mut_slice().iter_mut().enumerate() {
        *x = 4 - i as i32;
    }
    for (i, x) in v.as_slice().iter().enumerate() {
        assert_eq!(*x, 4 - i as i32);
    }
}

#[test]
fn push_pop() {
    let mut v = StaticVec::<i32, 4>::new();
    assert_eq!(v.len(), 0);

    for i in 0..3 {
        assert_eq!(v.push(i as i32), Ok(()));
    }
    assert_eq!(v.len(), 3);
    for i in 0..v.len() {
        assert_eq!(v[i], i as i32);
    }
    for (i, x) in v.iter().enumerate() {
        assert_eq!(*x, i as i32);
    }
    for (i, x) in v.as_slice().iter().enumerate() {
        assert_eq!(*x, i as i32);
    }

    assert_eq!(v.push(3), Ok(()));
    assert_eq!(v.push(4), Err(4));
    assert_eq!(v.len(), 4);
    for i in 0..v.len() {
        assert_eq!(v[i], i as i32);
    }
    for (i, x) in v.iter().enumerate() {
        assert_eq!(*x, i as i32);
    }
    for (i, x) in v.as_slice().iter().enumerate() {
        assert_eq!(*x, i as i32);
    }

    for i in 0..3 {
        assert_eq!(v.pop(), Some(3 - i as i32));
    }
    assert_eq!(v.len(), 1);
    for i in 0..v.len() {
        assert_eq!(v[i], i as i32);
    }
    for (i, x) in v.iter().enumerate() {
        assert_eq!(*x, i as i32);
    }
    for (i, x) in v.as_slice().iter().enumerate() {
        assert_eq!(*x, i as i32);
    }

    assert_eq!(v.pop(), Some(0));
    assert_eq!(v.pop(), None);
    assert_eq!(v.len(), 0);
}

#[test]
#[cfg(feature = "std")]
fn drop() {
    use std::{mem, rc::Rc};

    let (a, b) = (Rc::new(()), Rc::new(()));
    assert_eq!(Rc::strong_count(&a), 1);
    assert_eq!(Rc::strong_count(&b), 1);

    let mut v = StaticVec::<Rc<()>, 4>::new();
    v.push(a.clone()).unwrap();
    v.push(a.clone()).unwrap();
    v.push(b.clone()).unwrap();
    v.push(b.clone()).unwrap();
    v.push(b.clone()).unwrap_err();
    assert_eq!(Rc::strong_count(&a), 3);
    assert_eq!(Rc::strong_count(&b), 3);

    v.pop().unwrap();
    assert_eq!(Rc::strong_count(&a), 3);
    assert_eq!(Rc::strong_count(&b), 2);

    mem::drop(v);
    assert_eq!(Rc::strong_count(&a), 1);
    assert_eq!(Rc::strong_count(&b), 1);
}

#[test]
fn extend() {
    let mut v = StaticVec::<i32, 4>::new();
    v.extend_from_slice(&[0, 1, 2]);
    assert_eq!(v.len(), 3);
    for i in 0..3 {
        assert_eq!(v[i], i as i32);
    }

    v.extend_from_slice(&[3, 4]);
    assert_eq!(v.len(), 4);
    for i in 0..4 {
        assert_eq!(v[i], i as i32);
    }
}

#[test]
#[cfg(feature = "std")]
fn fmt() {
    use std::format;

    let mut v = StaticVec::<i32, 4>::new();
    assert_eq!(format!("{:?}", &v), "[]");

    v.extend_from_slice(&[0, 1, 2]);
    assert_eq!(format!("{:?}", &v), "[0, 1, 2]");

    v.push(3).unwrap();
    assert_eq!(format!("{:?}", &v), "[0, 1, 2, 3]");
}

#[test]
fn iter() {
    let v = StaticVec::<_, 4>::from_iter((0..3).into_iter());
    let mut it = v.into_iter();

    assert_eq!(it.next(), Some(0));
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), None);
}

#[test]
#[cfg(feature = "std")]
fn iter_drop() {
    use std::{mem, rc::Rc};

    let rcs = [Rc::new(()), Rc::new(()), Rc::new(())];
    let v = StaticVec::<_, 4>::from_slice(rcs.clone().as_ref());
    let mut it = v.into_iter();
    for rc in &rcs {
        assert_eq!(Rc::strong_count(rc), 2);
    }

    assert!(it.next().is_some());
    assert_eq!(Rc::strong_count(&rcs[0]), 1);
    assert_eq!(Rc::strong_count(&rcs[1]), 2);
    assert_eq!(Rc::strong_count(&rcs[2]), 2);

    mem::drop(it);
    for rc in &rcs {
        assert_eq!(Rc::strong_count(rc), 1);
    }
}

#[test]
fn remove() {
    let mut v = StaticVec::<_, 4>::from_iter((0..4).into_iter());

    assert_eq!(v.len(), 4);
    assert_eq!(v.remove(3), 3);
    assert_eq!(v.len(), 3);
    assert_eq!(v.remove(0), 0);
    assert_eq!(v.len(), 2);
    assert_eq!(v.remove(1), 2);
    assert_eq!(v.len(), 1);
    assert_eq!(v.remove(0), 1);
    assert_eq!(v.len(), 0);
}

#[test]
fn swap_remove() {
    let mut v = StaticVec::<_, 4>::from_iter((0..4).into_iter());

    assert_eq!(v.len(), 4);
    assert_eq!(v.swap_remove(0), 0);
    assert_eq!(v.len(), 3);
    assert_eq!(v.swap_remove(0), 3);
    assert_eq!(v.len(), 2);
    assert_eq!(v.swap_remove(1), 1);
    assert_eq!(v.len(), 1);
    assert_eq!(v.swap_remove(0), 2);
    assert_eq!(v.len(), 0);
}
