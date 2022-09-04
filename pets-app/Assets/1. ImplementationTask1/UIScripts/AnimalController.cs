using System.Collections;
using System.Collections.Generic;
using UnityEngine;

[RequireComponent(typeof(Rigidbody), typeof(BoxCollider))]
public class AnimalController : MonoBehaviour
{
    [SerializeField] private Rigidbody _rigyBody;
    private FixedJoystick _joyStick;
    [SerializeField] private Animator animator;

    [SerializeField] private float _moveSpeed;

    private void Awake()
    {
        _joyStick = GameObject.FindObjectOfType<FixedJoystick>();


    }


    // Update is called once per frame
    private void FixedUpdate()
    {
        if (_joyStick !=null){
            _rigyBody.velocity = new Vector3(-_joyStick.Horizontal * _moveSpeed, _rigyBody.velocity.y, -_joyStick.Vertical * _moveSpeed);
            if (_joyStick.Horizontal != 0 || _joyStick.Vertical != 0)
            {
                transform.rotation = Quaternion.LookRotation(_rigyBody.velocity);
                animator.SetBool("isRunning", true);
            }
            else
            {
                animator.SetBool("isRunning", false);
            }

        }
        else
        {
            _joyStick = GameObject.FindObjectOfType<FixedJoystick>();
        }

    }


}

