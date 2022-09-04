using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI; 

public class PlayWIthAnimal : MonoBehaviour
{

    public Animator animator;
    public GameObject SitBtn;
    public GameObject StandBtn; 
    public void Awake()
    {
        animator = GameObject.FindObjectOfType<Animator>();

    }

    public void Update()
    {
        if(animator == null)
        {
            animator = GameObject.FindObjectOfType<Animator>();
        }
    }
    public void SitDown()
    {
        animator.SetBool("isSit", !animator.GetBool("isSit"));
        //SitBtn.SetActive(false);
        //StandBtn.SetActive(true); 
        

    }
    public void standUp()
    {
        //animator.SetBool("isSit", false);
        //SitBtn.SetActive(true);
        //StandBtn.SetActive(false);
    }


    public void AttractPaws()
    {
        animator.SetTrigger("isPawsTrig");
       // animator.SetBool("isAttcakPaws", !animator.GetBool("isAttcakPaws"));

    }
    public void AttracTail()
    {
        animator.SetTrigger("isTailingTrig");
      //  animator.SetBool("isAttackTail", !animator.GetBool("isAttackTail"));

    }
    public void Falling()
    {
        animator.SetTrigger("isFallingTrig");

        //animator.SetBool("isFalling", !animator.GetBool("isFalling"));

    }
}
